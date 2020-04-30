use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::vec::IntoIter;

use crate::prop::animation::*;
use crate::prop::color::ColorProperty;

mod animation;
mod color;

pub type ParseResult<T> = Result<T, String>;
pub type ValueResult<T> = Result<PropertyValue<T>, String>;

pub trait Token {}

pub trait ParseToken<Token>: Parser {
    fn parse_token(&mut self) -> ValueResult<Token>;

    fn parse_full_token(&mut self) -> ValueResult<Token> {
        self.skip_white();
        self.consume_colon()?;
        self.skip_white();
        let _x = self.peek().cloned().unwrap_or_default();
        let _y = 1;
        let p = match self.try_parse_variable() {
            Some(v) => Ok(PropertyValue::Variable(v)),
            _ => self.parse_token(),
        };
        self.skip_white();
        self.consume_semicolon()?;
        p
    }
}

#[derive(Debug)]
pub struct CssTokenizer<'l> {
    it: Peekable<Chars<'l>>,
    tokens: Vec<String>,
    buffer: String,
}

impl<'l> CssTokenizer<'l> {
    pub fn new(s: &'l str) -> Self {
        Self {
            it: s.chars().into_iter().peekable(),
            tokens: vec![],
            buffer: "".to_string(),
        }
    }

    pub fn tokenize(mut self) -> Vec<String> {
        let mut escaped = false;
        while let Some(c) = self.it.next() {
            match c {
                '{' | '}' | ';' | ':' | ')' | '(' | '"' | '\'' | ',' | '%' => {
                    self.push_buffer();
                    self.tokens.push(c.to_string())
                }
                '\\' if !escaped => {
                    escaped = true;
                }
                '\\' if escaped => {
                    self.buffer.push('\\');
                }
                ' ' | '\n' if !escaped => {
                    self.push_buffer();
                    self.tokens.push(c.to_string());
                }
                ' ' | '\n' if escaped => {
                    self.buffer.push('\\');
                    self.buffer.push(c);
                }
                _ => {
                    self.buffer.push(c);
                }
            }
        }
        self.push_buffer();
        self.tokens
    }

    pub fn push_buffer(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let mut old = String::new();
        std::mem::swap(&mut self.buffer, &mut old);
        self.tokens.push(old);
    }
}

#[derive(Debug)]
pub struct ParserPosition {
    line: usize,
    line_character: usize,
    character: usize,
}

impl std::fmt::Display for ParserPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}:{}", self.line, self.line_character, self.character).as_str())
    }
}

pub trait Parser {
    fn consume(&mut self) -> Option<String>;
    fn expect_consume(&mut self) -> Result<String, String>;
    fn peek(&mut self) -> Option<&String>;
    fn skip_white(&mut self);
    fn consume_expected(&mut self, expected: &str) -> Result<String, String>;
    fn consume_semicolon(&mut self) -> Result<String, String>;
    fn consume_colon(&mut self) -> Result<String, String>;
    fn try_parse_variable(&mut self) -> Option<VariableUsage>;
}

#[derive(Debug)]
pub struct CssParser {
    tokens: Vec<String>,
    it: Peekable<IntoIter<String>>,
    selectors: Vec<Selector>,
    pos: ParserPosition,
    current: String,
    filename: String,
}

impl Parser for CssParser {
    fn consume(&mut self) -> Option<String> {
        let current = self.it.next();
        if let Some(s) = current.as_ref() {
            match s.as_str() {
                "\n" => {
                    self.pos.character += s.len();
                    self.pos.line += 1;
                    self.pos.line_character = 1;
                }
                _ => {
                    self.pos.character += s.len();
                    self.pos.line_character += s.len();
                }
            }
        }
        self.current = current.as_ref().cloned().unwrap_or_default();
        current
    }

    fn expect_consume(&mut self) -> Result<String, String> {
        self.consume()
            .ok_or_else(|| "expect token but nothing was found".to_string())
    }

    fn peek(&mut self) -> Option<&String> {
        self.it.peek()
    }

    fn skip_white(&mut self) {
        while let Some(s) = self.peek() {
            if !s.trim().is_empty() {
                break;
            }
            self.consume();
        }
    }

    fn consume_expected(&mut self, expected: &str) -> Result<String, String> {
        self.consume()
            .ok_or_else(|| {
                format!(
                    "expect to have parsable token {:?} but nothing was found",
                    expected
                )
            })
            .and_then(|s| {
                if s.as_str() == expected {
                    Ok(s)
                } else {
                    Err(format!(
                        "expect to find token {:?} but found {:?}",
                        expected, s
                    ))
                }
            })
    }

    fn consume_semicolon(&mut self) -> Result<String, String> {
        self.consume_expected(";")
    }

    fn consume_colon(&mut self) -> Result<String, String> {
        self.consume_expected(":")
    }

    fn try_parse_variable(&mut self) -> Option<VariableUsage> {
        let next = self.peek().cloned().unwrap_or_default();
        if "var" == next.as_str() {
            let v: ValueResult<VariableUsage> = self.parse_token();
            if let Ok(PropertyValue::Variable(usage)) = v {
                Some(usage)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl CssParser {
    pub fn new<S>(filename: S, tokens: Vec<String>) -> Self
    where
        S: Into<String>,
    {
        Self {
            tokens: tokens.clone(),
            it: tokens.into_iter().peekable(),
            selectors: vec![],
            pos: ParserPosition {
                line: 1,
                line_character: 1,
                character: 0,
            },
            current: "".to_string(),
            filename: filename.into(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Selector>, String> {
        while let Some(_) = self.peek() {
            let selector = self
                .parse_selector()
                .map_err(|error| self.failed_here(error))?;
            self.selectors.push(selector);
            self.skip_white();
        }
        Ok(self.selectors)
    }

    fn parse_selector(&mut self) -> Result<Selector, String> {
        let mut path = vec![];

        self.parse_selector_path(&mut path)?;
        self.skip_white();
        let block = self
            .expect_consume()
            .and_then(|s| self.parse_block(&path, s.as_str()))?;

        Ok(Selector { path, block })
    }

    fn parse_selector_path(&mut self, path: &mut Vec<SelectorPart>) -> Result<(), String> {
        while let Ok(PropertyValue::Other(part)) = self.parse_token() {
            path.push(part);
        }
        Ok(())
    }

    fn parse_block(&mut self, path: &Vec<SelectorPart>, token: &str) -> Result<Block, String> {
        if token != "{" {
            return Err(format!("expect to find block but found {:?}", token));
        }
        let mut block = Block { properties: vec![] };
        self.skip_white();
        while let Some(token) = self.consume() {
            match token.as_str() {
                "}" => break,
                "/*" => {
                    let mut comment = String::new();
                    while let Some(token) = self.peek() {
                        if token.as_str() == "*/" {
                            self.expect_consume()?;
                            self.skip_white();
                            break;
                        }
                        comment.push_str(self.expect_consume()?.as_str());
                    }
                    block.properties.push(Property::CommentBlock(comment))
                }
                _ => {
                    let prop = self.parse_property(path, token.as_str())?;
                    block.properties.push(prop);
                    self.skip_white();
                }
            };
        }
        Ok(block)
    }

    fn parse_property(&mut self, path: &Vec<SelectorPart>, s: &str) -> Result<Property, String> {
        let prop = match s {
            "align-content" => Property::AlignContent(self.parse_full_token()?),
            "align-items" => Property::AlignItems(self.parse_full_token()?),
            "align-self" => Property::AlignSelf(self.parse_full_token()?),
            "all" => Property::All(self.parse_full_token()?),
            "animation" => Property::Animation(self.parse_full_token()?),
            "animation-delay" => Property::AnimationDelay(self.parse_full_token()?),
            "animation-direction" => Property::AnimationDirection(self.parse_full_token()?),
            "animation-duration" => Property::AnimationDuration(self.parse_full_token()?),
            "animation-fill-mode" => Property::AnimationFillMode(self.parse_full_token()?),
            "animation-iteration-count" => {
                Property::AnimationIterationCount(self.parse_full_token()?)
            }
            "animation-name" => {
                self.skip_white();
                self.consume_colon()?;
                self.skip_white();
                let p = match self.try_parse_variable() {
                    Some(p) => PropertyValue::Variable(p),
                    _ => PropertyValue::Other(self.expect_consume()?),
                };
                self.consume_semicolon()?;
                Property::AnimationName(p)
            }
            "animation-play-state" => Property::AnimationPlayState(self.parse_full_token()?),
            "animation-timing-function" => {
                Property::AnimationTimingFunction(self.parse_full_token()?)
            }
            "backface-visibility" => Property::BackfaceVisibility(self.parse_full_token()?),
            //     "background" => Property::Background,
            "background-attachment" => Property::BackgroundAttachment(self.parse_full_token()?),
            "background-blend-mode" => Property::BackgroundBlendMode(self.parse_full_token()?),
            "background-clip" => Property::BackgroundClip(self.parse_full_token()?),
            "background-color" => Property::BackgroundColor(self.parse_full_token()?),
            //     "background-image" => Property::BackgroundImage,
            //     "background-origin" => Property::BackgroundOrigin,
            //     "background-position" => Property::BackgroundPosition,
            //     "background-repeat" => Property::BackgroundRepeat,
            //     "background-size" => Property::BackgroundSize,
            //     "border" => Property::Border,
            //     "border-bottom" => Property::BorderBottom,
            //     "border-bottom-color" => Property::BorderBottomColor,
            //     "border-bottom-left-radius" => Property::BorderBottomLeftRadius,
            //     "border-bottom-right-radius" => Property::BorderBottomRightRadius,
            //     "border-bottom-style" => Property::BorderBottomStyle,
            //     "border-bottom-width" => Property::BorderBottomWidth,
            //     "border-collapse" => Property::BorderCollapse,
            //     "border-color" => Property::BorderColor,
            //     "border-image" => Property::BorderImage,
            //     "border-image-outset" => Property::BorderImageOutset,
            //     "border-image-repeat" => Property::BorderImageRepeat,
            //     "border-image-slice" => Property::BorderImageSlice,
            //     "border-image-source" => Property::BorderImageSource,
            //     "border-image-width" => Property::BorderImageWidth,
            //     "border-left" => Property::BorderLeft,
            //     "border-left-color" => Property::BorderLeftColor,
            //     "border-left-style" => Property::BorderLeftStyle,
            //     "border-left-width" => Property::BorderLeftWidth,
            //     "border-radius" => Property::BorderRadius,
            //     "border-right" => Property::BorderRight,
            //     "border-right-color" => Property::BorderRightColor,
            //     "border-right-style" => Property::BorderRightStyle,
            //     "border-right-width" => Property::BorderRightWidth,
            //     "border-spacing" => Property::BorderSpacing,
            //     "border-style" => Property::BorderStyle,
            //     "border-top" => Property::BorderTop,
            //     "border-top-color" => Property::BorderTopColor,
            //     "border-top-left-radius" => Property::BorderTopLeftRadius,
            //     "border-top-right-radius" => Property::BorderTopRightRadius,
            //     "border-top-style" => Property::BorderTopStyle,
            //     "border-top-width" => Property::BorderTopWidth,
            //     "border-width" => Property::BorderWidth,
            //     "bottom" => Property::Bottom,
            //     "box-decoration-break" => Property::BoxDecorationBreak,
            //     "box-shadow" => Property::BoxShadow,
            //     "box-sizing" => Property::BoxSizing,
            //     "break-after" => Property::BreakAfter,
            //     "break-before" => Property::BreakBefore,
            //     "break-inside" => Property::BreakInside,
            //     "caption-side" => Property::CaptionSide,
            //     "caret-color" => Property::CaretColor,
            //     "@charset" => Property::AtCharset,
            "clear" => Property::Clear(self.parse_full_token()?),
            //     "clip" => Property::Clip,
            //     "clip-path" => Property::ClipPath,
            "color" => Property::Color(self.parse_full_token()?),
            "column-count" => Property::ColumnCount(self.parse_full_token()?),
            //     "column-fill" => Property::ColumnFill,
            //     "column-gap" => Property::ColumnGap,
            //     "column-rule" => Property::ColumnRule,
            //     "column-rule-color" => Property::ColumnRuleColor,
            //     "column-rule-style" => Property::ColumnRuleStyle,
            //     "column-rule-width" => Property::ColumnRuleWidth,
            //     "column-span" => Property::ColumnSpan,
            //     "column-width" => Property::ColumnWidth,
            //     "columns" => Property::Columns,
            //     "content" => Property::Content,
            //     "counter-increment" => Property::CounterIncrement,
            //     "counter-reset" => Property::CounterReset,
            //     "cursor" => Property::Cursor,
            //     "direction" => Property::Direction,
            "display" => Property::Display(self.parse_full_token()?),
            //     "empty-cells" => Property::EmptyCells,
            //     "filter" => Property::Filter,
            //     "flex" => Property::Flex,
            //     "flex-basis" => Property::FlexBasis,
            //     "flex-direction" => Property::FlexDirection,
            //     "flex-flow" => Property::FlexFlow,
            //     "flex-grow" => Property::FlexGrow,
            //     "flex-shrink" => Property::FlexShrink,
            //     "flex-wrap" => Property::FlexWrap,
            "float" => Property::Float(self.parse_full_token()?),
            //     "font" => Property::Font,
            //     "@font-face" => Property::AtFontFace,
            //     "font-family" => Property::FontFamily,
            //     "font-feature-settings" => Property::FontFeatureSettings,
            //     "font-kerning" => Property::FontKerning,
            //     "font-size" => Property::FontSize,
            //     "font-size-adjust" => Property::FontSizeAdjust,
            //     "font-stretch" => Property::FontStretch,
            //     "font-style" => Property::FontStyle,
            //     "font-variant" => Property::FontVariant,
            //     "font-variant-caps" => Property::FontVariantCaps,
            //     "font-weight" => Property::FontWeight,
            //     "grid" => Property::Grid,
            //     "grid-area" => Property::GridArea,
            //     "grid-auto-columns" => Property::GridAutoColumns,
            //     "grid-auto-flow" => Property::GridAutoFlow,
            //     "grid-auto-rows" => Property::GridAutoRows,
            //     "grid-column" => Property::GridColumn,
            //     "grid-column-end" => Property::GridColumnEnd,
            //     "grid-column-gap" => Property::GridColumnGap,
            //     "grid-column-start" => Property::GridColumnStart,
            //     "grid-gap" => Property::GridGap,
            //     "grid-row" => Property::GridRow,
            //     "grid-row-end" => Property::GridRowEnd,
            //     "grid-row-gap" => Property::GridRowGap,
            //     "grid-row-start" => Property::GridRowStart,
            //     "grid-template" => Property::GridTemplate,
            //     "grid-template-areas" => Property::GridTemplateAreas,
            //     "grid-template-columns" => Property::GridTemplateColumns,
            //     "grid-template-rows" => Property::GridTemplateRows,
            //     "hanging-punctuation" => Property::HangingPunctuation,
            //     "height" => Property::Height,
            //     "hyphens" => Property::Hyphens,
            //     "@import" => Property::AtImport,
            //     "isolation" => Property::Isolation,
            "justify-content" => Property::JustifyContent(self.parse_full_token()?),
            //     "@keyframes" => Property::AtKeyframes,
            //     "left" => Property::Left,
            //     "letter-spacing" => Property::LetterSpacing,
            //     "line-height" => Property::LineHeight,
            //     "list-style" => Property::ListStyle,
            //     "list-style-image" => Property::ListStyleImage,
            //     "list-style-position" => Property::ListStylePosition,
            //     "list-style-type" => Property::ListStyleType,
            //     "margin" => Property::Margin,
            //     "margin-bottom" => Property::MarginBottom,
            //     "margin-left" => Property::MarginLeft,
            //     "margin-right" => Property::MarginRight,
            //     "margin-top" => Property::MarginTop,
            //     "max-height" => Property::MaxHeight,
            //     "max-width" => Property::MaxWidth,
            //     "@media" => Property::AtMedia,
            //     "min-height" => Property::MinHeight,
            //     "min-width" => Property::MinWidth,
            //     "mix-blend-mode" => Property::MixBlendMode,
            //     "object-fit" => Property::ObjectFit,
            //     "object-position" => Property::ObjectPosition,
            //     "opacity" => Property::Opacity,
            //     "order" => Property::Order,
            //     "outline" => Property::Outline,
            //     "outline-color" => Property::OutlineColor,
            //     "outline-offset" => Property::OutlineOffset,
            //     "outline-style" => Property::OutlineStyle,
            //     "outline-width" => Property::OutlineWidth,
            //     "overflow" => Property::Overflow,
            //     "overflow-x" => Property::OverflowX,
            //     "overflow-y" => Property::OverflowY,
            //     "padding" => Property::Padding,
            //     "padding-bottom" => Property::PaddingBottom,
            //     "padding-left" => Property::PaddingLeft,
            //     "padding-right" => Property::PaddingRight,
            //     "padding-top" => Property::PaddingTop,
            //     "page-break-after" => Property::PageBreakAfter,
            //     "page-break-before" => Property::PageBreakBefore,
            //     "page-break-inside" => Property::PageBreakInside,
            //     "perspective" => Property::Perspective,
            //     "perspective-origin" => Property::PerspectiveOrigin,
            //     "pointer-events" => Property::PointerEvents,
            "position" => Property::Position(self.parse_full_token()?),
            //     "quotes" => Property::Quotes,
            //     "resize" => Property::Resize,
            //     "right" => Property::Right,
            //     "scroll-behavior" => Property::ScrollBehavior,
            //     "tab-size" => Property::TabSize,
            //     "table-layout" => Property::TableLayout,
            //     "text-align" => Property::TextAlign,
            //     "text-align-last" => Property::TextAlignLast,
            //     "text-decoration" => Property::TextDecoration,
            //     "text-decoration-color" => Property::TextDecorationColor,
            //     "text-decoration-line" => Property::TextDecorationLine,
            //     "text-decoration-style" => Property::TextDecorationStyle,
            //     "text-indent" => Property::TextIndent,
            //     "text-justify" => Property::TextJustify,
            //     "text-overflow" => Property::TextOverflow,
            //     "text-shadow" => Property::TextShadow,
            //     "text-transform" => Property::TextTransform,
            //     "top" => Property::Top,
            //     "transform" => Property::Transform,
            //     "transform-origin" => Property::TransformOrigin,
            //     "transform-style" => Property::TransformStyle,
            //     "transition" => Property::Transition,
            //     "transition-delay" => Property::TransitionDelay,
            //     "transition-duration" => Property::TransitionDuration,
            //     "transition-property" => Property::TransitionProperty,
            //     "transition-timing-function" => Property::TransitionTimingFunction,
            //     "unicode-bidi" => Property::UnicodeBidi,
            //     "user-select" => Property::UserSelect,
            //     "vertical-align" => Property::VerticalAlign,
            //     "visibility" => Property::Visibility,
            //     "white-space" => Property::WhiteSpace,
            //     "width" => Property::Width,
            //     "word-break" => Property::WordBreak,
            //     "word-spacing" => Property::WordSpacing,
            //     "word-wrap" => Property::WordWrap,
            //     "writing-mode" => Property::WritingMode,
            "z-index" => Property::ZIndex(self.parse_full_token()?),
            _ if s.starts_with("--") && s.len() > 2 => {
                if path == &vec![SelectorPart::PseudoSelector("host".to_string())] {
                    let value: PropertyValue<String> = self.parse_full_token()?;
                    Property::VariableDefinition(s[2..].to_string(), value)
                } else {
                    return Err(format!("invalid property {:?}", s));
                }
            }
            _ => {
                let _x = 1;
                return Err(format!("invalid property {:?}", s));
            }
        };
        Ok(prop)
    }

    fn parse_expected<ExpectedType>(&mut self) -> Result<ExpectedType, String>
    where
        ExpectedType: FromStr<Err = String>,
    {
        let s = self.expect_consume()?;
        s.parse::<ExpectedType>()
    }

    fn next_is_semicolon(&mut self) -> bool {
        self.peek().map(|s| s.as_str() == ";").unwrap_or_default()
    }

    fn failed_here(&self, error: String) -> String {
        let cwd = std::env::current_dir().unwrap();
        let relative = cwd.join(self.filename.as_str());
        let p = match relative.canonicalize() {
            Ok(p) => p,
            _ => relative,
        }
        .display()
        .to_string();
        format!("{} {}:{}", error, p, self.pos)
    }
}

#[derive(Debug, PartialEq)]
pub struct Selector {
    path: Vec<SelectorPart>,
    block: Block,
}

#[derive(Debug, PartialEq)]
pub enum SelectorPart {
    Class(String),
    Id(String),
    Attr(String, String),
    TagName(String),
    PseudoFunc(Box<SelectorPart>),
    PseudoSelector(String),
    Any,
    // >
    ParentBound,
    // +
    AfterSiblingBound,
    // ~
    BeforeSiblingBound,
}

impl ParseToken<SelectorPart> for CssParser {
    fn parse_token(&mut self) -> ValueResult<SelectorPart> {
        let mut buffer = String::new();
        self.skip_white();
        while let Some(s) = self.peek().cloned() {
            match s.as_str() {
                "{" if buffer.is_empty() => return Err("end of selector".to_string()),
                "{" | ">" | "~" | "+" | ":" | "(" | "[" | "#" | "." if !buffer.is_empty() => {
                    return Ok(PropertyValue::Other(buffer.parse::<SelectorPart>()?));
                }
                _ if !buffer.is_empty() && s.starts_with(".") | s.starts_with("#") => {
                    return Ok(PropertyValue::Other(buffer.parse::<SelectorPart>()?));
                }
                _ => {
                    self.expect_consume()?;
                    buffer.push_str(s.as_str())
                }
            };
            self.skip_white();
        }
        if buffer.is_empty() {
            Err("".to_string())
        } else {
            Ok(PropertyValue::Other(buffer.parse::<SelectorPart>()?))
        }
    }
}

impl FromStr for SelectorPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("invalid selector".to_string());
        }
        let _x = 1;
        let p = match s {
            ">" => SelectorPart::ParentBound,
            "+" => SelectorPart::AfterSiblingBound,
            "~" => SelectorPart::BeforeSiblingBound,
            _ if s.starts_with(".") => SelectorPart::Class(s[1..].to_string()),
            _ if s.starts_with("#") => SelectorPart::Id(s[1..].to_string()),
            _ if s.starts_with(":") && !s.contains("(") => {
                SelectorPart::PseudoSelector(s[1..].to_string())
            }
            _ => SelectorPart::TagName(s.to_string()),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    properties: Vec<Property>,
}

#[derive(Debug, PartialEq)]
pub enum BlockProperty {
    Property(Box<Property>),
    Selector(Box<Selector>),
}

impl Token for usize {}

impl ParseToken<usize> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<usize>, String> {
        match self.try_parse_variable() {
            Some(p) => Ok(PropertyValue::Variable(p)),
            _ => {
                let count = self.expect_consume()?;
                Ok(PropertyValue::Other(count.parse().map_err(|_| {
                    format!("invalid token, expect number got {:?}", count)
                })?))
            }
        }
    }
}

impl Token for u8 {}

impl ParseToken<u8> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u8>, String> {
        match self.try_parse_variable() {
            Some(p) => Ok(PropertyValue::Variable(p)),
            _ => {
                let count = self.expect_consume()?;
                Ok(PropertyValue::Other(count.parse().map_err(|_| {
                    format!(
                        "invalid token, expect short number greater or equal 0 got {:?}",
                        count
                    )
                })?))
            }
        }
    }
}

impl Token for u16 {}

impl ParseToken<u16> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u16>, String> {
        match self.try_parse_variable() {
            Some(p) => Ok(PropertyValue::Variable(p)),
            _ => {
                let count = self.expect_consume()?;
                Ok(PropertyValue::Other(count.parse().map_err(|_| {
                    format!(
                        "invalid token, expect middle range number greater or equal 0 got {:?}",
                        count
                    )
                })?))
            }
        }
    }
}

impl Token for u32 {}

impl ParseToken<u32> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u32>, String> {
        match self.try_parse_variable() {
            Some(p) => Ok(PropertyValue::Variable(p)),
            _ => {
                let count = self.expect_consume()?;
                Ok(PropertyValue::Other(count.parse().map_err(|_| {
                    format!(
                        "invalid token, expect number greater or equal 0 got {:?}",
                        count
                    )
                })?))
            }
        }
    }
}

impl Token for f64 {}

impl ParseToken<f64> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<f64>, String> {
        match self.try_parse_variable() {
            Some(p) => Ok(PropertyValue::Variable(p)),
            _ => {
                let count = self.expect_consume()?;
                Ok(PropertyValue::Other(count.parse().map_err(|_| {
                    format!(
                        "invalid token, expect floating point number got {:?}",
                        count
                    )
                })?))
            }
        }
    }
}

impl PropertyValue<f64> {
    fn into_color_alpha(self) -> PropertyValue<u8> {
        match self {
            PropertyValue::Variable(v) => PropertyValue::Variable(v),
            PropertyValue::Other(f) => PropertyValue::Other((f * 255f64) as u8),
        }
    }
}

impl Token for String {}

impl ParseToken<String> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<String>, String> {
        if let Some(p) = self.try_parse_variable() {
            Ok(PropertyValue::Variable(p))
        } else {
            Ok(PropertyValue::Other(self.expect_consume()?))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimeProperty {
    Seconds(i32),
    MilliSeconds(i32),
    Initial,
    Inherit,
}

impl Token for TimeProperty {}

impl ParseToken<TimeProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<TimeProperty> {
        self.skip_white();
        let current = self.expect_consume()?;
        let p = match current.as_str() {
            "initial" => TimeProperty::Initial,
            "inherit" => TimeProperty::Inherit,
            _ if current.ends_with("ms") => match current[0..(current.len() - 2)].parse::<i32>() {
                Ok(n) => TimeProperty::MilliSeconds(n),
                _ => return Err(format!("invalid time {:?}", self.current)),
            },
            _ if current.ends_with("s") => match current[0..(current.len() - 1)].parse::<i32>() {
                Ok(n) => TimeProperty::Seconds(n),
                _ => return Err(format!("invalid time {:?}", self.current)),
            },
            _ => return Err(format!("invalid time {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum DisplayProperty {
    Inline,
    Block,
    Contents,
    Flex,
    Grid,
    InlineBlock,
    InlineFlex,
    InlineGrid,
    InlineTable,
    ListItem,
    RunIn,
    Table,
    TableCaption,
    TableColumnGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRowGroup,
    TableCell,
    TableColumn,
    TableRow,
    None,
    Initial,
    Inherit,
}

impl Token for DisplayProperty {}

impl ParseToken<DisplayProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<DisplayProperty>, String> {
        let p = match self.expect_consume()?.as_str() {
            "inline" => DisplayProperty::Inline,
            "block" => DisplayProperty::Block,
            "contents" => DisplayProperty::Contents,
            "flex" => DisplayProperty::Flex,
            "grid" => DisplayProperty::Grid,
            "inline-block" => DisplayProperty::InlineBlock,
            "inline-flex" => DisplayProperty::InlineFlex,
            "inline-grid" => DisplayProperty::InlineGrid,
            "inline-table" => DisplayProperty::InlineTable,
            "list-item" => DisplayProperty::ListItem,
            "run-in" => DisplayProperty::RunIn,
            "table" => DisplayProperty::Table,
            "table-caption" => DisplayProperty::TableCaption,
            "table-column-group" => DisplayProperty::TableColumnGroup,
            "table-header-group" => DisplayProperty::TableHeaderGroup,
            "table-footer-group" => DisplayProperty::TableFooterGroup,
            "table-row-group" => DisplayProperty::TableRowGroup,
            "table-cell" => DisplayProperty::TableCell,
            "table-column" => DisplayProperty::TableColumn,
            "table-row" => DisplayProperty::TableRow,
            "none" => DisplayProperty::None,
            "initial" => DisplayProperty::Initial,
            "inherit" => DisplayProperty::Inherit,
            _ => return Err(format!("invalid display value {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AlignContentProperty {
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    Initial,
    Inherit,
}

impl Token for AlignContentProperty {}

impl ParseToken<AlignContentProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AlignContentProperty>, String> {
        let p = match self.expect_consume()?.as_str() {
            "stretch" => AlignContentProperty::Stretch,
            "center" => AlignContentProperty::Center,
            "flex-start" => AlignContentProperty::FlexStart,
            "flex-end" => AlignContentProperty::FlexEnd,
            "space-between" => AlignContentProperty::SpaceBetween,
            "space-around" => AlignContentProperty::SpaceAround,
            "initial" => AlignContentProperty::Initial,
            "inherit" => AlignContentProperty::Inherit,
            _ => return Err(format!("invalid align-content {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AlignItemsProperty {
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    Baseline,
    Initial,
    Inherit,
}

impl Token for AlignItemsProperty {}

impl ParseToken<AlignItemsProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AlignItemsProperty>, String> {
        let p = match self.expect_consume()?.as_str() {
            "stretch" => AlignItemsProperty::Stretch,
            "center" => AlignItemsProperty::Center,
            "flex-start" => AlignItemsProperty::FlexStart,
            "flex-end" => AlignItemsProperty::FlexEnd,
            "baseline" => AlignItemsProperty::Baseline,
            "initial" => AlignItemsProperty::Initial,
            "inherit" => AlignItemsProperty::Inherit,
            _ => return Err(format!("invalid align-items {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AlignSelfProperty {
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    Baseline,
    Initial,
    Inherit,
    Auto,
}

impl Token for AlignSelfProperty {}

impl ParseToken<AlignSelfProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AlignSelfProperty>, String> {
        let p = match self.expect_consume()?.as_str() {
            "stretch" => AlignSelfProperty::Stretch,
            "center" => AlignSelfProperty::Center,
            "flex-start" => AlignSelfProperty::FlexStart,
            "flex-end" => AlignSelfProperty::FlexEnd,
            "baseline" => AlignSelfProperty::Baseline,
            "initial" => AlignSelfProperty::Initial,
            "inherit" => AlignSelfProperty::Inherit,
            "auto" => AlignSelfProperty::Auto,
            _ => return Err(format!("invalid align-self {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AllProperty {
    Initial,
    Inherit,
    Unset,
}

impl Token for AllProperty {}

impl ParseToken<AllProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AllProperty>, String> {
        let p = match self.expect_consume()?.as_str() {
            "initial" => AllProperty::Initial,
            "inherit" => AllProperty::Inherit,
            "unset" => AllProperty::Unset,
            _ => return Err(format!("invalid all {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum ColumnCountProperty {
    Auto,
    Number(u32),
}

impl Token for ColumnCountProperty {}

impl ParseToken<ColumnCountProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<ColumnCountProperty> {
        let p = match self.expect_consume()?.as_str() {
            "auto" => ColumnCountProperty::Auto,
            s @ _ => ColumnCountProperty::Number(
                s.parse::<u32>()
                    .map_err(|_| format!("invalid column count"))?,
            ),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum JustifyContentProperty {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Initial,
    Inherit,
}

impl Token for JustifyContentProperty {}

impl ParseToken<JustifyContentProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<JustifyContentProperty>, String> {
        let p = match self.expect_consume()?.to_lowercase().as_str() {
            "flex-start" => JustifyContentProperty::FlexStart,
            "flex-end" => JustifyContentProperty::FlexEnd,
            "center" => JustifyContentProperty::Center,
            "space-between" => JustifyContentProperty::SpaceBetween,
            "space-around" => JustifyContentProperty::SpaceAround,
            "initial" => JustifyContentProperty::Initial,
            "inherit" => JustifyContentProperty::Inherit,
            _ => return Err(format!("invalid justify-content {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum BackfaceVisibilityProperty {
    Visible,
    Hidden,
    Initial,
    Inherit,
}

impl ParseToken<BackfaceVisibilityProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<BackfaceVisibilityProperty> {
        let p = match self.expect_consume()?.as_str() {
            "visible" => BackfaceVisibilityProperty::Visible,
            "hidden" => BackfaceVisibilityProperty::Hidden,
            "initial" => BackfaceVisibilityProperty::Initial,
            "inherit" => BackfaceVisibilityProperty::Inherit,
            _ => return Err(format!("invalid backface visibility {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum ZIndexProperty {
    Auto,
    Number(i32),
    Initial,
    Inherit,
}

impl ParseToken<ZIndexProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<ZIndexProperty> {
        let p = match self.expect_consume()?.as_str() {
            "auto" => ZIndexProperty::Auto,
            "initial" => ZIndexProperty::Initial,
            "inherit" => ZIndexProperty::Inherit,
            _ => ZIndexProperty::Number(
                self.current
                    .parse::<i32>()
                    .map_err(|_| format!("invalid z-index {:?}", self.current))?,
            ),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum ClearProperty {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,

    Initial,
    Inherit,
    Unset,
}

impl Token for ClearProperty {}

impl ParseToken<ClearProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<ClearProperty> {
        let p = match self.expect_consume()?.as_str() {
            "none" => ClearProperty::None,
            "left" => ClearProperty::Left,
            "right" => ClearProperty::Right,
            "both" => ClearProperty::Both,
            "inline-start" => ClearProperty::InlineStart,
            "inline-end" => ClearProperty::InlineEnd,
            "initial" => ClearProperty::Initial,
            "inherit" => ClearProperty::Inherit,
            "unset" => ClearProperty::Unset,
            _ => return Err(format!("invalid clear {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum BackgroundClipProperty {
    BorderBox,
    PaddingBox,
    ContentBox,
    Initial,
    Inherit,
}

impl Token for BackgroundClipProperty {}

impl ParseToken<BackgroundClipProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<BackgroundClipProperty> {
        let p = match self.expect_consume()?.as_str() {
            "border-box" => BackgroundClipProperty::BorderBox,
            "padding-box" => BackgroundClipProperty::PaddingBox,
            "content-box" => BackgroundClipProperty::ContentBox,
            "initial" => BackgroundClipProperty::Initial,
            "inherit" => BackgroundClipProperty::Inherit,
            _ => return Err(format!("invalid background clip {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum BackgroundBlendModeProperty {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    Saturation,
    Color,
    Luminosity,
}

impl Token for BackgroundBlendModeProperty {}

impl ParseToken<BackgroundBlendModeProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<BackgroundBlendModeProperty> {
        let p = match self.expect_consume()?.as_str() {
            "normal" => BackgroundBlendModeProperty::Normal,
            "multiply" => BackgroundBlendModeProperty::Multiply,
            "screen" => BackgroundBlendModeProperty::Screen,
            "overlay" => BackgroundBlendModeProperty::Overlay,
            "darken" => BackgroundBlendModeProperty::Darken,
            "lighten" => BackgroundBlendModeProperty::Lighten,
            "color-dodge" => BackgroundBlendModeProperty::ColorDodge,
            "saturation" => BackgroundBlendModeProperty::Saturation,
            "color" => BackgroundBlendModeProperty::Color,
            "luminosity" => BackgroundBlendModeProperty::Luminosity,
            _ => return Err(format!("invalid background blend mode {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum BackgroundDirectionProperty {
    ToRight,
    ToLeft,
    ToTop,
    ToBottom,
    Angle(PropertyValue<usize>),
}

#[derive(Debug, PartialEq)]
pub enum BackgroundAttachmentProperty {
    Scroll,
    Fixed,
    Local,
    Initial,
    Inherit,
}

impl Token for BackgroundAttachmentProperty {}

impl ParseToken<BackgroundAttachmentProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<BackgroundAttachmentProperty> {
        let p = match self.expect_consume()?.as_str() {
            "scroll" => BackgroundAttachmentProperty::Scroll,
            "fixed" => BackgroundAttachmentProperty::Fixed,
            "local" => BackgroundAttachmentProperty::Local,
            "initial" => BackgroundAttachmentProperty::Initial,
            "inherit" => BackgroundAttachmentProperty::Inherit,
            _ => return Err(format!("invalid background attachment {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AngleUnit {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
}

#[derive(Debug, PartialEq)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Token for PositionProperty {}

impl ParseToken<PositionProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<PositionProperty> {
        let p = match self.expect_consume()?.as_str() {
            "static" => PositionProperty::Static,
            "relative" => PositionProperty::Relative,
            "absolute" => PositionProperty::Absolute,
            "fixed" => PositionProperty::Fixed,
            "sticky" => PositionProperty::Sticky,
            _ => return Err(format!("invalid position {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum FloatProperty {
    Left,
    Right,
    None,
    InlineStart,
    InlineEnd,
}

impl Token for FloatProperty {}

impl ParseToken<FloatProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<FloatProperty> {
        let p = match self.expect_consume()?.as_str() {
            "left" => FloatProperty::Left,
            "right" => FloatProperty::Right,
            "none" => FloatProperty::None,
            "inline-start" => FloatProperty::InlineStart,
            "inline-end" => FloatProperty::InlineEnd,
            _ => return Err(format!("invalid float {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableUsage(String);

impl Token for VariableUsage {}

impl ParseToken<VariableUsage> for CssParser {
    fn parse_token(&mut self) -> ValueResult<VariableUsage> {
        if self.expect_consume()?.as_str() != "var" {
            return Err(format!(
                "value does not start with var keyword {:?}",
                self.current
            ));
        }
        self.skip_white();
        self.consume_expected("(")?;
        self.skip_white();
        let s = self.expect_consume()?;
        if !s.starts_with("--") {
            return Err(format!(
                "invalid variable name {:?}, variable must start with --",
                s
            ));
        }
        self.skip_white();
        self.consume_expected(")")?;
        self.skip_white();
        Ok(PropertyValue::Variable(VariableUsage(s[2..].to_string())))
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyValue<T> {
    Variable(VariableUsage),
    Other(T),
}

#[derive(Debug, PartialEq)]
pub enum Property {
    AlignContent(PropertyValue<AlignContentProperty>),
    AlignItems(PropertyValue<AlignItemsProperty>),
    AlignSelf(PropertyValue<AlignSelfProperty>),
    All(PropertyValue<AllProperty>),
    Animation(PropertyValue<AnimationProperty>),
    AnimationDelay(PropertyValue<AnimationDelayProperty>),
    AnimationDirection(PropertyValue<AnimationDirectionProperty>),
    AnimationDuration(PropertyValue<TimeProperty>),
    AnimationFillMode(PropertyValue<AnimationFillModeProperty>),
    AnimationIterationCount(PropertyValue<usize>),
    AnimationName(PropertyValue<String>),
    AnimationPlayState(PropertyValue<AnimationPlayStateProperty>),
    AnimationTimingFunction(PropertyValue<AnimationTimingFunctionProperty>),
    BackfaceVisibility(PropertyValue<BackfaceVisibilityProperty>),
    Background(String),
    BackgroundAttachment(PropertyValue<BackgroundAttachmentProperty>),
    BackgroundBlendMode(PropertyValue<BackgroundBlendModeProperty>),
    BackgroundClip(PropertyValue<BackgroundClipProperty>),
    BackgroundColor(PropertyValue<ColorProperty>),
    BackgroundImage(String),
    BackgroundOrigin(String),
    BackgroundPosition(String),
    BackgroundRepeat(String),
    BackgroundSize(String),
    Border(String),
    BorderBottom(String),
    BorderBottomColor(String),
    BorderBottomLeftRadius(String),
    BorderBottomRightRadius(String),
    BorderBottomStyle(String),
    BorderBottomWidth(String),
    BorderCollapse(String),
    BorderColor(String),
    BorderImage(String),
    BorderImageOutset(String),
    BorderImageRepeat(String),
    BorderImageSlice(String),
    BorderImageSource(String),
    BorderImageWidth(String),
    BorderLeft(String),
    BorderLeftColor(String),
    BorderLeftStyle(String),
    BorderLeftWidth(String),
    BorderRadius(String),
    BorderRight(String),
    BorderRightColor(String),
    BorderRightStyle(String),
    BorderRightWidth(String),
    BorderSpacing(String),
    BorderStyle(String),
    BorderTop(String),
    BorderTopColor(String),
    BorderTopLeftRadius(String),
    BorderTopRightRadius(String),
    BorderTopStyle(String),
    BorderTopWidth(String),
    BorderWidth(String),
    Bottom(String),
    BoxDecorationBreak(String),
    BoxShadow(String),
    BoxSizing(String),
    BreakAfter(String),
    BreakBefore(String),
    BreakInside(String),
    CaptionSide(String),
    CaretColor(String),
    AtCharset(String),
    Clear(PropertyValue<ClearProperty>),
    Clip(String),
    ClipPath(String),
    Color(PropertyValue<ColorProperty>),
    ColumnCount(PropertyValue<ColumnCountProperty>),
    ColumnFill(String),
    ColumnGap(String),
    ColumnRule(String),
    ColumnRuleColor(String),
    ColumnRuleStyle(String),
    ColumnRuleWidth(String),
    ColumnSpan(String),
    ColumnWidth(String),
    Columns(String),
    Content(String),
    CounterIncrement(String),
    CounterReset(String),
    Cursor(String),
    Direction(String),
    Display(PropertyValue<DisplayProperty>),
    EmptyCells(String),
    Filter(String),
    Flex(String),
    FlexBasis(String),
    FlexDirection(String),
    FlexFlow(String),
    FlexGrow(String),
    FlexShrink(String),
    FlexWrap(String),
    Float(PropertyValue<FloatProperty>),
    Font(String),
    AtFontFace(String),
    FontFamily(String),
    FontFeatureSettings(String),
    FontKerning(String),
    FontSize(String),
    FontSizeAdjust(String),
    FontStretch(String),
    FontStyle(String),
    FontVariant(String),
    FontVariantCaps(String),
    FontWeight(String),
    Grid(String),
    GridArea(String),
    GridAutoColumns(String),
    GridAutoFlow(String),
    GridAutoRows(String),
    GridColumn(String),
    GridColumnEnd(String),
    GridColumnGap(String),
    GridColumnStart(String),
    GridGap(String),
    GridRow(String),
    GridRowEnd(String),
    GridRowGap(String),
    GridRowStart(String),
    GridTemplate(String),
    GridTemplateAreas(String),
    GridTemplateColumns(String),
    GridTemplateRows(String),
    HangingPunctuation(String),
    Height(String),
    Hyphens(String),
    AtImport(String),
    Isolation(String),
    JustifyContent(PropertyValue<JustifyContentProperty>),
    AtKeyframes(String),
    Left(String),
    LetterSpacing(String),
    LineHeight(String),
    ListStyle(String),
    ListStyleImage(String),
    ListStylePosition(String),
    ListStyleType(String),
    Margin(String),
    MarginBottom(String),
    MarginLeft(String),
    MarginRight(String),
    MarginTop(String),
    MaxHeight(String),
    MaxWidth(String),
    AtMedia(String),
    MinHeight(String),
    MinWidth(String),
    MixBlendMode(String),
    ObjectFit(String),
    ObjectPosition(String),
    Opacity(String),
    Order(String),
    Outline(String),
    OutlineColor(String),
    OutlineOffset(String),
    OutlineStyle(String),
    OutlineWidth(String),
    Overflow(String),
    OverflowX(String),
    OverflowY(String),
    Padding(String),
    PaddingBottom(String),
    PaddingLeft(String),
    PaddingRight(String),
    PaddingTop(String),
    PageBreakAfter(String),
    PageBreakBefore(String),
    PageBreakInside(String),
    Perspective(String),
    PerspectiveOrigin(String),
    PointerEvents(String),
    Position(PropertyValue<PositionProperty>),
    Quotes(String),
    Resize(String),
    Right(String),
    ScrollBehavior(String),
    TabSize(String),
    TableLayout(String),
    TextAlign(String),
    TextAlignLast(String),
    TextDecoration(String),
    TextDecorationColor(String),
    TextDecorationLine(String),
    TextDecorationStyle(String),
    TextIndent(String),
    TextJustify(String),
    TextOverflow(String),
    TextShadow(String),
    TextTransform(String),
    Top(String),
    Transform(String),
    TransformOrigin(String),
    TransformStyle(String),
    Transition(String),
    TransitionDelay(String),
    TransitionDuration(String),
    TransitionProperty(String),
    TransitionTimingFunction(String),
    UnicodeBidi(String),
    UserSelect(String),
    VerticalAlign(String),
    Visibility(String),
    WhiteSpace(String),
    Width(String),
    WordBreak(String),
    WordSpacing(String),
    WordWrap(String),
    WritingMode(String),
    ZIndex(PropertyValue<ZIndexProperty>),
    // special
    CommentBlock(String),
    VariableDefinition(String, PropertyValue<String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Selector {
        pub fn new(path: Vec<SelectorPart>, props: Vec<Property>) -> Self {
            Self {
                path,
                block: Block { properties: props },
            }
        }
    }

    /// we assume currently we hit property name
    /// display : block;
    /// ^
    /// so we need to add `:`
    ///
    /// But we also we adding spaces around because they are allowed in css and needs to be skipped
    fn parse_prop_value(s: &str) -> CssParser {
        let full = format!(" : {} {}", s, if s.contains(";") { "" } else { ";" });
        let tokens = CssTokenizer::new(full.as_str()).tokenize();
        CssParser::new("", tokens)
    }

    fn parse_simple_value(s: &str) -> CssParser {
        let tokens = CssTokenizer::new(s).tokenize();
        CssParser::new("", tokens)
    }

    #[test]
    fn parse_prop_value_variable() {
        let res: Result<PropertyValue<VariableUsage>, String> =
            parse_simple_value("var(--a)").parse_token();
        let expected = Ok(PropertyValue::Variable(VariableUsage("a".to_string())));
        assert_eq!(res, expected);
        let res: Result<PropertyValue<VariableUsage>, String> =
            parse_simple_value("var(--foo)").parse_token();
        let expected = Ok(PropertyValue::Variable(VariableUsage("foo".to_string())));
        assert_eq!(res, expected);
        let res: Result<PropertyValue<VariableUsage>, String> =
            parse_simple_value("--foo").parse_token();
        let expected = Err(r#"given string is not a variable"--foo""#.to_string());
        assert_eq!(res, expected);
        let res: Result<PropertyValue<VariableUsage>, String> =
            parse_simple_value("foo").parse_token();
        let expected = Err(r#"given string is not a variable"foo""#.to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn parse_color() {
        let res: ValueResult<ColorProperty> = parse_simple_value("#123").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(17),
            PropertyValue::Other(34),
            PropertyValue::Other(51),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("#010203").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(1),
            PropertyValue::Other(2),
            PropertyValue::Other(3),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("#fff").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(255),
            PropertyValue::Other(255),
            PropertyValue::Other(255),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("#ffffff").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(255),
            PropertyValue::Other(255),
            PropertyValue::Other(255),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("#abcdef").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(171),
            PropertyValue::Other(205),
            PropertyValue::Other(239),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("currentColor").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Current));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("rgb(1,2,3)").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(1),
            PropertyValue::Other(2),
            PropertyValue::Other(3),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("rgb(1,2,3,.2)").parse_token();
        let expected = Err("expect to find token \")\" but found \",\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("rgba(1,2,3,.1)").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(1),
            PropertyValue::Other(2),
            PropertyValue::Other(3),
            PropertyValue::Other(25),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> =
            parse_simple_value("hsla(100,20%,30%,.4)").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Hsla(
            PropertyValue::Other(100),
            PropertyValue::Other(20),
            PropertyValue::Other(30),
            PropertyValue::Other(0.4f64),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("hsl(100,20%,30%)").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Hsla(
            PropertyValue::Other(100),
            PropertyValue::Other(20),
            PropertyValue::Other(30),
            PropertyValue::Other(1.0f64),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> =
            parse_simple_value("hsl(100,20%,30%,0.5)").parse_token();
        let expected = Err("expect to find token \")\" but found \",\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("CornflowerBlue").parse_token();
        let expected = Ok(PropertyValue::Other(ColorProperty::Rgba(
            PropertyValue::Other(100),
            PropertyValue::Other(149),
            PropertyValue::Other(237),
            PropertyValue::Other(255),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<ColorProperty> = parse_simple_value("foo").parse_token();
        let expected = Err("\"foo\" is not a predefined color".to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn parse_selector_part() {
        let res = "".parse::<SelectorPart>();
        let expected = Err("invalid selector".to_string());
        assert_eq!(res, expected);
    }

    #[test]
    fn parse_time() {
        let res: ValueResult<TimeProperty> = parse_prop_value("").parse_token();
        let expected = Err("invalid time".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("as").parse_token();
        let expected = Err("invalid time".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("3s").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::Seconds(3)));
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("2s").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::Seconds(2)));
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("-5s").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::Seconds(-5)));
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("3ms").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::MilliSeconds(3)));
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("2ms").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::MilliSeconds(2)));
        assert_eq!(res, expected);
        let res: ValueResult<TimeProperty> = parse_prop_value("-5ms").parse_token();
        let expected = Ok(PropertyValue::Other(TimeProperty::MilliSeconds(-5)));
        assert_eq!(res, expected);
    }

    #[test]
    fn tokenize_empty_p_selector() {
        let source = r#"p{}"#;
        let result = CssTokenizer::new(source).tokenize();
        assert_eq!(
            result,
            vec!["p".to_string(), "{".to_string(), "}".to_string()],
        )
    }

    #[test]
    fn tokenize_empty_div_selector() {
        let source = r#"div{}"#;
        let result = CssTokenizer::new(source).tokenize();
        assert_eq!(
            result,
            vec!["div".to_string(), "{".to_string(), "}".to_string()],
        )
    }

    #[test]
    fn tokenize_empty_custom_selector() {
        let source = r#"app-user{}"#;
        let result = CssTokenizer::new(source).tokenize();
        assert_eq!(
            result,
            vec!["app-user".to_string(), "{".to_string(), "}".to_string()],
        )
    }

    #[test]
    fn tokenize_p_display_block() {
        let source = r#"p { display: block; }"#;
        let result = CssTokenizer::new(source).tokenize();
        assert_eq!(
            result,
            vec![
                "p".to_string(),
                " ".to_string(),
                "{".to_string(),
                " ".to_string(),
                "display".to_string(),
                ":".to_string(),
                " ".to_string(),
                "block".to_string(),
                ";".to_string(),
                " ".to_string(),
                "}".to_string()
            ],
        )
    }

    #[test]
    fn tokenize_p_background_image_url() {
        let source = r#"p { background-image: url("hello/world.jpg"); }"#;
        let result = CssTokenizer::new(source).tokenize();
        assert_eq!(
            result,
            vec![
                "p".to_string(),
                " ".to_string(),
                "{".to_string(),
                " ".to_string(),
                "background-image".to_string(),
                ":".to_string(),
                " ".to_string(),
                "url".to_string(),
                "(".to_string(),
                "\"".to_string(),
                "hello/world.jpg".to_string(),
                "\"".to_string(),
                ")".to_string(),
                ";".to_string(),
                " ".to_string(),
                "}".to_string()
            ],
        )
    }

    #[test]
    fn parse_empty_p_selector() {
        let source = r#"p{}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block { properties: vec![] },
            }]),
        )
    }

    #[test]
    fn parse_p_align_content() {
        let source = r#"p { align-content: flex-start; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![Property::AlignContent(PropertyValue::Other(
                        AlignContentProperty::FlexStart
                    )),]
                },
            }]),
        )
    }

    #[test]
    fn parse_p_display_block() {
        let source = r#"p { display: block; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![Property::Display(PropertyValue::Other(
                        DisplayProperty::Block
                    ))]
                },
            }]),
        )
    }

    #[test]
    fn parse_long_path() {
        let source = r#"p p {}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::TagName("p".to_string())
                ],
                block: Block { properties: vec![] },
            }]),
        )
    }

    #[test]
    fn parse_long_path_with_parent_bound() {
        let source = r#"p > p {}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::ParentBound,
                    SelectorPart::TagName("p".to_string()),
                ],
                block: Block { properties: vec![] },
            }]),
        )
    }

    #[test]
    fn parse_long_path_with_before_sibling_bound() {
        let source = r#"p ~ p {}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::BeforeSiblingBound,
                    SelectorPart::TagName("p".to_string()),
                ],
                block: Block { properties: vec![] },
            }]),
        )
    }

    #[test]
    fn parse_long_path_with_after_sibling_bound() {
        let source = r#"p + p {}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::AfterSiblingBound,
                    SelectorPart::TagName("p".to_string()),
                ],
                block: Block { properties: vec![] },
            }]),
        )
    }

    #[test]
    fn parse_multiple_properties() {
        let source = r#"p { display: block; justify-content: initial; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string()),],
                block: Block {
                    properties: vec![
                        Property::Display(PropertyValue::Other(DisplayProperty::Block)),
                        Property::JustifyContent(PropertyValue::Other(
                            JustifyContentProperty::Initial
                        )),
                    ]
                },
            }]),
        )
    }

    #[test]
    fn parse_animation() {
        let source = r#"p { animation: initial; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![Property::Animation(PropertyValue::Other(
                        AnimationProperty::Initial
                    ))]
                },
            }]),
        );
        let source = r#"p { animation: inherit; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![Property::Animation(PropertyValue::Other(
                        AnimationProperty::Inherit
                    )),]
                },
            }]),
        );
        let source = r#"p { animation: some; }"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new("", tokens).parse();
        let animation = Property::Animation(PropertyValue::Other(AnimationProperty::Custom(
            "some".to_string(),
            PropertyValue::Other(TimeProperty::Seconds(0)),
            PropertyValue::Other(AnimationTimingFunctionProperty::Ease),
            PropertyValue::Other(AnimationDelayProperty::Time(PropertyValue::Other(
                TimeProperty::Seconds(0),
            ))),
            PropertyValue::Other(1),
            PropertyValue::Other(AnimationDirectionProperty::Normal),
            PropertyValue::Other(AnimationFillModeProperty::None),
            PropertyValue::Other(AnimationPlayStateProperty::Running),
        )));
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![animation]
                },
            }]),
        );
    }

    #[test]
    fn test_file_a() {
        let tokens = CssTokenizer::new(include_str!("../../tests/a.css")).tokenize();
        let res = CssParser::new("./tests/a.css", tokens).parse();
        let expected: Result<Vec<Selector>, String> = Ok(vec![
            Selector {
                path: vec![SelectorPart::Class("foo".to_string())],
                block: Block { properties: vec![] },
            },
            Selector {
                path: vec![SelectorPart::Class("bar".to_string())],
                block: Block { properties: vec![] },
            },
            Selector::new(vec![SelectorPart::Class("foz".to_string())], vec![]),
            Selector::new(
                vec![SelectorPart::Class("baz".to_string())],
                vec![
                    Property::Display(PropertyValue::Other(DisplayProperty::Block)),
                    Property::JustifyContent(PropertyValue::Other(
                        JustifyContentProperty::SpaceBetween,
                    )),
                    Property::Color(PropertyValue::Other(ColorProperty::Rgba(
                        PropertyValue::Other(255),
                        PropertyValue::Other(0),
                        PropertyValue::Other(0),
                        PropertyValue::Other(255),
                    ))),
                    Property::BackgroundColor(PropertyValue::Other(ColorProperty::Rgba(
                        PropertyValue::Other(66),
                        PropertyValue::Other(65),
                        PropertyValue::Other(61),
                        PropertyValue::Other(255),
                    ))),
                ],
            ),
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_file_full() {
        use animation::*;
        use Property::*;
        use PropertyValue::*;

        let tokens = CssTokenizer::new(include_str!("../../tests/full.css")).tokenize();
        let res = CssParser::new("./jirs-css/tests/full.css", tokens).parse();
        let expected = vec![
            Selector::new(
                vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::ParentBound,
                    SelectorPart::Id("foo".to_string()),
                    SelectorPart::AfterSiblingBound,
                    SelectorPart::Class("bar".to_string()),
                ],
                vec![
                    AlignContent(Other(AlignContentProperty::SpaceBetween)),
                    AlignItems(Other(AlignItemsProperty::Center)),
                    AlignSelf(Other(AlignSelfProperty::Auto)),
                    All(Other(AllProperty::Inherit)),
                    Animation(Other(AnimationProperty::Custom(
                        "slidein".to_string(),
                        Other(TimeProperty::Seconds(3)),
                        Other(AnimationTimingFunctionProperty::EaseIn),
                        Other(AnimationDelayProperty::Time(Other(TimeProperty::Seconds(
                            1,
                        )))),
                        Other(2),
                        Other(AnimationDirectionProperty::Reverse),
                        Other(AnimationFillModeProperty::Both),
                        Other(AnimationPlayStateProperty::Paused),
                    ))),
                    AnimationDelay(Other(AnimationDelayProperty::Time(Other(
                        TimeProperty::MilliSeconds(4),
                    )))),
                    AnimationDirection(Other(AnimationDirectionProperty::AlternateReverse)),
                    AnimationDuration(Other(TimeProperty::Seconds(5))),
                    AnimationIterationCount(Other(9)),
                    AnimationName(Other("my-animation".to_string())),
                    AnimationPlayState(Other(AnimationPlayStateProperty::Paused)),
                    AnimationTimingFunction(Other(AnimationTimingFunctionProperty::EaseIn)),
                    BackfaceVisibility(Other(BackfaceVisibilityProperty::Visible)),
                    BackgroundAttachment(Other(BackgroundAttachmentProperty::Local)),
                    BackgroundBlendMode(Other(BackgroundBlendModeProperty::Darken)),
                    BackgroundClip(Other(BackgroundClipProperty::ContentBox)),
                    BackgroundColor(Other(ColorProperty::Rgba(
                        Other(12),
                        Other(34),
                        Other(56),
                        Other(153),
                    ))),
                    CommentBlock(" ".to_string()),
                    Clear(Other(ClearProperty::Left)),
                    Color(Other(ColorProperty::Rgba(
                        Other(0),
                        Other(255),
                        Other(255),
                        Other(255),
                    ))),
                    Display(Other(DisplayProperty::Block)),
                    Float(Other(FloatProperty::Right)),
                    JustifyContent(Other(JustifyContentProperty::FlexEnd)),
                    Position(Other(PositionProperty::Relative)),
                    ZIndex(Other(ZIndexProperty::Number(-2))),
                ],
            ),
            Selector::new(
                vec![
                    SelectorPart::TagName("p".to_string()),
                    SelectorPart::ParentBound,
                    SelectorPart::Id("foo".to_string()),
                    SelectorPart::BeforeSiblingBound,
                    SelectorPart::Class("bar".to_string()),
                ],
                vec![
                    AlignContent(Variable(VariableUsage("align-content".to_string()))),
                    AlignItems(Variable(VariableUsage("align-items".to_string()))),
                    AlignSelf(Variable(VariableUsage("align-self".to_string()))),
                    All(Variable(VariableUsage("all".to_string()))),
                    Animation(Variable(VariableUsage("animation".to_string()))),
                    AnimationDelay(Variable(VariableUsage("animation-delay".to_string()))),
                    AnimationDirection(Variable(VariableUsage("animation-direction".to_string()))),
                    AnimationDuration(Variable(VariableUsage("animation-duration".to_string()))),
                    AnimationIterationCount(Variable(VariableUsage(
                        "animation-iteration-count".to_string(),
                    ))),
                    AnimationName(Variable(VariableUsage("animation-name".to_string()))),
                    AnimationPlayState(Variable(VariableUsage("animation-play-state".to_string()))),
                    AnimationTimingFunction(Variable(VariableUsage(
                        "animation-timing-function".to_string(),
                    ))),
                    BackfaceVisibility(Variable(VariableUsage("backface-visibility".to_string()))),
                    BackgroundAttachment(Variable(VariableUsage(
                        "background-attachment".to_string(),
                    ))),
                    BackgroundBlendMode(Variable(VariableUsage(
                        "background-blend-mode".to_string(),
                    ))),
                    BackgroundClip(Variable(VariableUsage("background-clip".to_string()))),
                    BackgroundColor(Variable(VariableUsage("background-color".to_string()))),
                    CommentBlock(" ".to_string()),
                    Clear(Variable(VariableUsage("clear".to_string()))),
                    Color(Variable(VariableUsage("color".to_string()))),
                    Display(Variable(VariableUsage("display".to_string()))),
                    Float(Variable(VariableUsage("float".to_string()))),
                    JustifyContent(Variable(VariableUsage("justify-content".to_string()))),
                    Position(Variable(VariableUsage("position".to_string()))),
                    ZIndex(Variable(VariableUsage("z-index".to_string()))),
                ],
            ),
            Selector::new(
                vec![SelectorPart::PseudoSelector("host".to_string())],
                vec![
                    Property::VariableDefinition(
                        "clear".to_string(),
                        PropertyValue::Other("clear".to_string()),
                    ),
                    Property::VariableDefinition(
                        "from-var".to_string(),
                        PropertyValue::Variable(VariableUsage("clear".to_string())),
                    ),
                ],
            ),
        ];
        if let Ok(v) = res.as_ref() {
            for (idx, selector) in v.into_iter().enumerate() {
                let expected_selector = expected
                    .get(idx)
                    .expect("expectation does not have enough selectors");
                assert_eq!(selector.path, expected_selector.path);
                for (prop_idx, prop) in selector.block.properties.iter().enumerate() {
                    let expected_prop = expected_selector.block.properties.get(prop_idx).expect(
                        format!(
                            "expectation does not have enough properties {} {}",
                            idx, prop_idx
                        )
                        .as_str(),
                    );
                    assert_eq!(prop, expected_prop);
                }
            }
            assert_eq!(res, Ok(expected));
        } else {
            assert_eq!(res, Ok(expected));
        }
    }
}
