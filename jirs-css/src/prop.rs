use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::vec::IntoIter;

use crate::colors::Color;

pub type ParseResult<T> = Result<T, String>;
pub type ValueResult<T> = Result<PropertyValue<T>, String>;

pub trait Token {}

pub trait ParseToken<Token> {
    fn parse_token(&mut self) -> ValueResult<Token>;
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

#[derive(Debug)]
pub struct CssParser {
    tokens: Vec<String>,
    it: Peekable<IntoIter<String>>,
    selectors: Vec<Selector>,
    pos: ParserPosition,
    current: String,
    filename: String,
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
        while let Some(token) = self.consume() {
            let selector = self
                .parse_selector(token.as_str())
                .map_err(|error| format!("{} ({}:{})", error, self.filename, self.pos))?;
            self.selectors.push(selector);
            self.skip_white();
        }
        Ok(self.selectors)
    }

    fn parse_selector(&mut self, token: &str) -> Result<Selector, String> {
        let mut path = vec![];

        if let Ok(part) = token.parse::<SelectorPart>() {
            path.push(part);
            self.parse_selector_path(&mut path)?;
        }
        self.skip_white();
        let block = self
            .expect_consume()
            .and_then(|s| self.parse_block(s.as_str()))?;

        Ok(Selector { path, block })
    }

    fn parse_selector_path(&mut self, path: &mut Vec<SelectorPart>) -> Result<(), String> {
        self.skip_white();
        while let Some(token) = self.peek() {
            if token.as_str() == "{" {
                break;
            }
            match self.expect_consume()?.parse::<SelectorPart>() {
                Ok(part) => {
                    path.push(part);
                }
                _ => break,
            };
            self.skip_white();
        }
        Ok(())
    }

    fn parse_block(&mut self, token: &str) -> Result<Block, String> {
        if token != "{" {
            return Err(format!("expect to find block but found {:?}", token));
        }
        let mut block = Block { properties: vec![] };
        self.skip_white();
        while let Some(token) = self.consume() {
            if token.as_str() == "}" {
                break;
            }
            let prop = self.parse_property(token.as_str())?;
            block.properties.push(prop);
            self.skip_white();
        }
        Ok(block)
    }

    fn parse_property(&mut self, s: &str) -> Result<Property, String> {
        let prop = match s {
            "align-content" => Property::AlignContent(self.parse_token()?),
            "align-items" => Property::AlignItems(self.parse_token()?),
            "align-self" => Property::AlignSelf(self.parse_token()?),
            "all" => Property::All(self.parse_token()?),
            "animation" => Property::Animation(self.parse_token()?),
            "animation-delay" => Property::AnimationDelay(self.parse_token()?),
            "animation-direction" => Property::AnimationDirection(self.parse_token()?),
            "animation-duration" => Property::AnimationDuration(self.parse_token()?),
            "animation-fill-mode" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::AnimationFillMode(p)
            }
            "animation-iteration-count" => Property::AnimationIterationCount(self.parse_token()?),
            "animation-name" => Property::AnimationName(self.parse_token()?),
            "animation-play-state" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::AnimationPlayState(p)
            }
            "animation-timing-function" => {
                let p = self.parse_token()?;
                self.consume_expected(";")?;
                Property::AnimationTimingFunction(p)
            }
            "backface-visibility" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::BackfaceVisibility(p)
            }
            //     "background" => Property::Background,
            "background-attachment" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::BackgroundAttachment(p)
            }
            "background-blend-mode" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::BackgroundBlendMode(p)
            }
            "background-clip" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::BackgroundClip(p)
            }
            "background-color" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_token()?;
                self.consume_expected(";")?;
                Property::BackgroundColor(p)
            }
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
            "clear" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::Clear(p)
            }
            //     "clip" => Property::Clip,
            //     "clip-path" => Property::ClipPath,
            "color" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_token()?;
                self.consume_expected(";")?;
                Property::Color(p)
            }
            //     "column-count" => Property::ColumnCount,
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
            "display" => Property::Display(self.parse_token()?),
            //     "empty-cells" => Property::EmptyCells,
            //     "filter" => Property::Filter,
            //     "flex" => Property::Flex,
            //     "flex-basis" => Property::FlexBasis,
            //     "flex-direction" => Property::FlexDirection,
            //     "flex-flow" => Property::FlexFlow,
            //     "flex-grow" => Property::FlexGrow,
            //     "flex-shrink" => Property::FlexShrink,
            //     "flex-wrap" => Property::FlexWrap,
            "float" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::Float(p)
            }
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
            "justify-content" => Property::JustifyContent(self.parse_token()?),
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
            "position" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::Position(p)
            }
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
            "z-index" => {
                let p = self
                    .parse_expected_prop_value()
                    .map_err(|_| format!("invalid z-index, expect number got {:?}", s))?;
                self.consume_expected(";")?;
                Property::ZIndex(p)
            }
            _ => {
                let _x = 1;
                return Err(format!("invalid property {:?}", s));
            }
        };
        Ok(prop)
    }

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

    fn parse_expected<ExpectedType>(&mut self) -> Result<ExpectedType, String>
    where
        ExpectedType: FromStr<Err = String>,
    {
        let s = self.expect_consume()?;
        s.parse::<ExpectedType>()
    }

    fn parse_expected_prop_value<ExpectedType>(&mut self) -> ValueResult<ExpectedType>
    where
        ExpectedType: FromStr<Err = String>,
    {
        let s = self.expect_consume()?;
        if Self::check_if_variable(s.as_str()) {
            Self::parse_prop_value_variable(s.as_str())
        } else {
            s.parse::<ExpectedType>().map(|v| PropertyValue::Other(v))
        }
    }

    fn current_is_semicolon(&mut self) -> bool {
        self.peek().map(|s| s.as_str() == ";").unwrap_or_default()
    }

    fn parse_prop_value_variable<T>(s: &str) -> Result<PropertyValue<T>, String> {
        if !Self::check_if_variable(s) {
            return Err(format!("given string is not a variable {:?}", s));
        }
        Ok(PropertyValue::Variable(
            s[6..(s.len() - 1)].trim().to_string(),
        ))
    }

    fn check_if_variable(s: &str) -> bool {
        s.starts_with("var(--") && s.ends_with(")")
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

impl FromStr for SelectorPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err("invalid selector".to_string());
        }
        let s = match s {
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
        Ok(s)
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
        let count = self.expect_consume()?;
        let p = if Self::check_if_variable(count.as_str()) {
            Self::parse_prop_value_variable(count.as_str())?
        } else {
            PropertyValue::Other(
                count
                    .parse()
                    .map_err(|_| format!("invalid token, expect number got {:?}", count))?,
            )
        };

        Ok(p)
    }
}

impl Token for u8 {}

impl ParseToken<u8> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u8>, String> {
        let count = self.expect_consume()?;
        let p = if Self::check_if_variable(count.as_str()) {
            Self::parse_prop_value_variable(count.as_str())?
        } else {
            PropertyValue::Other(count.parse().map_err(|_| {
                format!(
                    "invalid token, expect short number greater or equal 0 got {:?}",
                    count
                )
            })?)
        };

        Ok(p)
    }
}

impl Token for u16 {}

impl ParseToken<u16> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u16>, String> {
        let count = self.expect_consume()?;
        let p = if Self::check_if_variable(count.as_str()) {
            Self::parse_prop_value_variable(count.as_str())?
        } else {
            PropertyValue::Other(count.parse().map_err(|_| {
                format!(
                    "invalid token, expect middle range number greater or equal 0 got {:?}",
                    count
                )
            })?)
        };

        Ok(p)
    }
}

impl Token for u32 {}

impl ParseToken<u32> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<u32>, String> {
        let count = self.expect_consume()?;
        let p = if Self::check_if_variable(count.as_str()) {
            Self::parse_prop_value_variable(count.as_str())?
        } else {
            PropertyValue::Other(count.parse().map_err(|_| {
                format!(
                    "invalid token, expect number greater or equal 0 got {:?}",
                    count
                )
            })?)
        };

        Ok(p)
    }
}

impl Token for f64 {}

impl ParseToken<f64> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<f64>, String> {
        let count = self.expect_consume()?;
        let p = if Self::check_if_variable(count.as_str()) {
            Self::parse_prop_value_variable(count.as_str())?
        } else {
            PropertyValue::Other(count.parse().map_err(|_| {
                format!(
                    "invalid token, expect floating point number got {:?}",
                    count
                )
            })?)
        };

        Ok(p)
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
        self.consume_expected(":")?;
        let name = match self.expect_consume()?.as_str() {
            name @ _ if Self::check_if_variable(name) => Self::parse_prop_value_variable(name)?,
            name @ _ => PropertyValue::Other(name.to_string()),
        };
        Ok(name)
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
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected::<DisplayProperty>()?;
        self.consume_expected(";")?;
        Ok(PropertyValue::Other(p))
    }
}

impl FromStr for DisplayProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match s {
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
            _ => return Err("invalid display".to_string()),
        };
        Ok(d)
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
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected_prop_value()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for AlignContentProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "stretch" => AlignContentProperty::Stretch,
            "center" => AlignContentProperty::Center,
            "flex-start" => AlignContentProperty::FlexStart,
            "flex-end" => AlignContentProperty::FlexEnd,
            "space-between" => AlignContentProperty::SpaceBetween,
            "space-around" => AlignContentProperty::SpaceAround,
            "initial" => AlignContentProperty::Initial,
            "inherit" => AlignContentProperty::Inherit,
            _ => return Err("invalid align-content".to_string()),
        };
        Ok(p)
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
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected_prop_value()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for AlignItemsProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "stretch" => AlignItemsProperty::Stretch,
            "center" => AlignItemsProperty::Center,
            "flex-start" => AlignItemsProperty::FlexStart,
            "flex-end" => AlignItemsProperty::FlexEnd,
            "baseline" => AlignItemsProperty::Baseline,
            "initial" => AlignItemsProperty::Initial,
            "inherit" => AlignItemsProperty::Inherit,
            _ => return Err("invalid align-items".to_string()),
        };
        Ok(p)
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
}

impl Token for AlignSelfProperty {}

impl ParseToken<AlignSelfProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AlignSelfProperty>, String> {
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected_prop_value::<AlignSelfProperty>()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for AlignSelfProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "stretch" => AlignSelfProperty::Stretch,
            "center" => AlignSelfProperty::Center,
            "flex-start" => AlignSelfProperty::FlexStart,
            "flex-end" => AlignSelfProperty::FlexEnd,
            "baseline" => AlignSelfProperty::Baseline,
            "initial" => AlignSelfProperty::Initial,
            "inherit" => AlignSelfProperty::Inherit,
            _ => return Err("invalid align-self".to_string()),
        };
        Ok(p)
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
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected_prop_value::<AllProperty>()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for AllProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "initial" => AllProperty::Initial,
            "inherit" => AllProperty::Inherit,
            "unset" => AllProperty::Unset,
            _ => return Err("invalid all".to_string()),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationDirectionProperty {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
    Initial,
    Inherit,
}

impl Token for AnimationDirectionProperty {}

impl ParseToken<AnimationDirectionProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AnimationDirectionProperty>, String> {
        let p = self.parse_expected_prop_value::<AnimationDirectionProperty>()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for AnimationDirectionProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "normal" => AnimationDirectionProperty::Normal,
            "reverse" => AnimationDirectionProperty::Reverse,
            "alternate" => AnimationDirectionProperty::Alternate,
            "alternate-reverse" => AnimationDirectionProperty::AlternateReverse,
            "initial" => AnimationDirectionProperty::Initial,
            "inherit" => AnimationDirectionProperty::Inherit,
            _ => return Err(format!("invalid animation direction {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationFillModeProperty {
    None,
    Forwards,
    Backwards,
    Both,
    Initial,
    Inherit,
}

impl FromStr for AnimationFillModeProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "none" => AnimationFillModeProperty::None,
            "forwards" => AnimationFillModeProperty::Forwards,
            "backwards" => AnimationFillModeProperty::Backwards,
            "both" => AnimationFillModeProperty::Both,
            "initial" => AnimationFillModeProperty::Initial,
            "inherit" => AnimationFillModeProperty::Inherit,
            _ => return Err(format!("invalid animation fill mode {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationPlayStateProperty {
    Paused,
    Running,
    Initial,
    Inherit,
}

impl FromStr for AnimationPlayStateProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "paused" => AnimationPlayStateProperty::Paused,
            "running" => AnimationPlayStateProperty::Running,
            "initial" => AnimationPlayStateProperty::Initial,
            "inherit" => AnimationPlayStateProperty::Inherit,
            _ => return Err(format!("invalid animation play state {:?}", s)),
        };
        Ok(p)
    }
}

pub enum Animation {}

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
        self.consume_expected(":")?;
        self.skip_white();
        let current = self.expect_consume()?;
        let p = match current.as_str() {
            "initial" => TimeProperty::Initial,
            "inherit" => TimeProperty::Inherit,
            _ if current.ends_with("ms") => match current[0..(current.len() - 2)].parse::<i32>() {
                Ok(n) => TimeProperty::MilliSeconds(n),
                _ => return Err("invalid time".to_string()),
            },
            _ if current.ends_with("s") => match current[0..(current.len() - 1)].parse::<i32>() {
                Ok(n) => TimeProperty::Seconds(n),
                _ => return Err("invalid time".to_string()),
            },
            _ => return Err("invalid time".to_string()),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimingFunctionSteps {
    Start,
    End,
}

impl Token for AnimationTimingFunctionSteps {}

impl ParseToken<AnimationTimingFunctionSteps> for CssParser {
    fn parse_token(&mut self) -> ValueResult<AnimationTimingFunctionSteps> {
        let s = self.expect_consume()?;
        let p = match s.to_lowercase().as_str() {
            "start" => AnimationTimingFunctionSteps::Start,
            "end" => AnimationTimingFunctionSteps::End,
            _ => return Err(format!("invalid animation timing function step {:?}", s)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    Steps(
        PropertyValue<u32>,
        PropertyValue<AnimationTimingFunctionSteps>,
    ),
    CubicBezier(
        PropertyValue<f64>,
        PropertyValue<f64>,
        PropertyValue<f64>,
        PropertyValue<f64>,
    ),
    Initial,
    Inherit,
}

impl Token for AnimationTimingFunction {}

impl ParseToken<AnimationTimingFunction> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AnimationTimingFunction>, String> {
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let current = self.expect_consume()?;
        let p = match current.as_str() {
            "linear" => AnimationTimingFunction::Linear,
            "ease" => AnimationTimingFunction::Ease,
            "ease-in" => AnimationTimingFunction::EaseIn,
            "ease-out" => AnimationTimingFunction::EaseOut,
            "ease-in-out" => AnimationTimingFunction::EaseInOut,
            "step-start" => AnimationTimingFunction::StepStart,
            "step-end" => AnimationTimingFunction::StepEnd,
            "initial" => AnimationTimingFunction::Initial,
            "inherit" => AnimationTimingFunction::Inherit,
            "steps" => {
                self.consume_expected("(")?;
                self.skip_white();
                let b = self.parse_token()?;
                match b {
                    PropertyValue::Other(n) if n <= 0 => {
                        return Err(format!("invalid animation timing function, number of iterations must be greater than 0"));
                    }
                    _ => (),
                }
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let c = self.parse_token()?;
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                self.consume_expected(";")?;
                AnimationTimingFunction::Steps(b, c)
            }
            "cubic-bezier" => {
                self.consume_expected("(")?;
                self.skip_white();
                let a = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let b = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let c = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let d = self.parse_token()?;
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                self.consume_expected(";")?;
                AnimationTimingFunction::CubicBezier(a, b, c, d)
            }
            _ => return Err(format!("invalid animation timing function {:?}", current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationDelayProperty {
    Time(PropertyValue<TimeProperty>),
    Initial,
    Inherit,
}

impl Token for AnimationDelayProperty {}

impl ParseToken<AnimationDelayProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AnimationDelayProperty>, String> {
        self.skip_white();
        let current = self.expect_consume()?;
        let p = match current.as_str() {
            "initial" => AnimationDelayProperty::Initial,
            "inherit" => AnimationDelayProperty::Inherit,
            _ => AnimationDelayProperty::Time(self.parse_token()?),
        };
        self.consume_expected(";")?;
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationProperty {
    Initial,
    Inherit,
    Custom(
        String,
        PropertyValue<TimeProperty>,
        PropertyValue<AnimationTimingFunction>,
        PropertyValue<AnimationDelayProperty>,
        PropertyValue<usize>,
        PropertyValue<AnimationDirectionProperty>,
        PropertyValue<AnimationFillModeProperty>,
        PropertyValue<AnimationPlayStateProperty>,
    ),
}

impl Token for AnimationProperty {}

impl ParseToken<AnimationProperty> for CssParser {
    fn parse_token(&mut self) -> Result<PropertyValue<AnimationProperty>, String> {
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let def = self.expect_consume()?;
        let p = match def.as_str() {
            "initial" => PropertyValue::Other(AnimationProperty::Initial),
            "inherit" => PropertyValue::Other(AnimationProperty::Inherit),
            _ if Self::check_if_variable(def.as_str()) => {
                Self::parse_prop_value_variable(def.as_str())?
            }
            _ => {
                let name = def;
                self.skip_white();

                let duration = if self.current_is_semicolon() {
                    PropertyValue::Other(TimeProperty::Seconds(0))
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let timing = if self.current_is_semicolon() {
                    PropertyValue::Other(AnimationTimingFunction::Ease)
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let delay = if self.current_is_semicolon() {
                    PropertyValue::Other(AnimationDelayProperty::Time(PropertyValue::Other(
                        TimeProperty::Seconds(0),
                    )))
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let iteration_count = if self.current_is_semicolon() {
                    PropertyValue::Other(1)
                } else {
                    let count = self.expect_consume()?;
                    let v = count.parse::<usize>().map_err(|_| {
                        format!(
                            "invalid animation iteration count, expect number got {:?}",
                            count
                        )
                    })?;
                    self.skip_white();
                    PropertyValue::Other(v)
                };
                let direction = if self.current_is_semicolon() {
                    PropertyValue::Other(AnimationDirectionProperty::Normal)
                } else {
                    let v = self.parse_expected_prop_value::<AnimationDirectionProperty>()?;
                    self.skip_white();
                    v
                };
                let fill_mode = if self.current_is_semicolon() {
                    PropertyValue::Other(AnimationFillModeProperty::None)
                } else {
                    let v = self.parse_expected_prop_value::<AnimationFillModeProperty>()?;
                    self.skip_white();
                    v
                };
                let play_state = if self.current_is_semicolon() {
                    PropertyValue::Other(AnimationPlayStateProperty::Running)
                } else {
                    let v = self.parse_expected_prop_value::<AnimationPlayStateProperty>()?;
                    self.skip_white();
                    v
                };

                PropertyValue::Other(AnimationProperty::Custom(
                    name,
                    duration,
                    timing,
                    delay,
                    iteration_count,
                    direction,
                    fill_mode,
                    play_state,
                ))
            }
        };
        self.consume_expected(";")?;
        Ok(p)
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
        self.skip_white();
        self.consume_expected(":")?;
        self.skip_white();
        let p = self.parse_expected_prop_value::<JustifyContentProperty>()?;
        self.consume_expected(";")?;
        Ok(p)
    }
}

impl FromStr for JustifyContentProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s.to_lowercase().as_str() {
            "flex-start" => JustifyContentProperty::FlexStart,
            "flex-end" => JustifyContentProperty::FlexEnd,
            "center" => JustifyContentProperty::Center,
            "space-between" => JustifyContentProperty::SpaceBetween,
            "space-around" => JustifyContentProperty::SpaceAround,
            "initial" => JustifyContentProperty::Initial,
            "inherit" => JustifyContentProperty::Inherit,
            _ => return Err(format!("invalid justify-content {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum BackfaceVisibilityProperty {
    Visible,
    Hidden,
    Initial,
    Inherit,
}

impl FromStr for BackfaceVisibilityProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "visible" => BackfaceVisibilityProperty::Visible,
            "hidden" => BackfaceVisibilityProperty::Hidden,
            "initial" => BackfaceVisibilityProperty::Initial,
            "inherit" => BackfaceVisibilityProperty::Inherit,
            _ => return Err(format!("invalid backface visibility {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum ZIndexProperty {
    Auto,
    Number(i32),
    Initial,
    Inherit,
}

impl FromStr for ZIndexProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "auto" => ZIndexProperty::Auto,
            "initial" => ZIndexProperty::Initial,
            "inherit" => ZIndexProperty::Inherit,
            _ => ZIndexProperty::Number(
                s.parse::<i32>()
                    .map_err(|_| format!("invalid z-index {:?}", s))?,
            ),
        };
        Ok(p)
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

impl FromStr for ClearProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "none" => ClearProperty::None,
            "left" => ClearProperty::Left,
            "right" => ClearProperty::Right,
            "both" => ClearProperty::Both,
            "inline-start" => ClearProperty::InlineStart,
            "inline-end" => ClearProperty::InlineEnd,
            "initial" => ClearProperty::Initial,
            "inherit" => ClearProperty::Inherit,
            "unset" => ClearProperty::Unset,
            _ => return Err(format!("invalid clear {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum ColorProperty {
    Name(String),
    Rgba(
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<u8>,
    ),
    Hsla(
        PropertyValue<u16>,
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<f64>,
    ),
    Current,
}

impl Token for ColorProperty {}

impl ParseToken<ColorProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<ColorProperty> {
        self.skip_white();
        let current = self.expect_consume()?;
        let s = current.trim();

        let p = match s {
            "currentColor" => ColorProperty::Current,
            _ if s.len() == 7 && s.starts_with('#') => {
                let r = u8::from_str_radix(&s[1..=2], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[3..=4], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[5..=6], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(255),
                )
            }
            _ if s.len() == 4 && s.starts_with('#') => {
                let _x = &s[1..=1];
                let r = u8::from_str_radix(&s[1..=1].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[2..=2].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[3..=3].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(255),
                )
            }
            _ if s.len() == 9 && s.starts_with('#') => {
                let (r, g, b, a) = (
                    u8::from_str_radix(&s[1..=2], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[3..=4], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[5..=6], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[7..=8], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                );
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(a),
                )
            }
            "rgba" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let r = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let g = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let b = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let a = self.parse_token()?.into_color_alpha();
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Rgba(r, g, b, a)
            }
            "rgb" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let r = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let g = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let b = self.parse_token()?;
                self.skip_white();
                let a = PropertyValue::Other(255);
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Rgba(r, g, b, a)
            }
            "hsla" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let h = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let s = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let l = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let a = self.parse_token()?;
                match a {
                    PropertyValue::Other(f) if -0.001f64 > f || f > 1.001f64 => {
                        return Err(format!("out of range hsl alpha value {:?}", a))
                    }
                    _ => (),
                };
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Hsla(h, s, l, a)
            }
            "hsl" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let h = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let s = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let l = self.parse_token()?;
                self.consume_expected("%")?;
                let a = PropertyValue::Other(1f64);
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Hsla(h, s, l, a)
            }

            _ => s
                .parse::<Color>()
                .map(|c| c.to_values())
                .and_then(|(r, g, b)| {
                    Ok(ColorProperty::Rgba(
                        PropertyValue::Other(r),
                        PropertyValue::Other(g),
                        PropertyValue::Other(b),
                        PropertyValue::Other(255),
                    ))
                })?,
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

impl FromStr for BackgroundClipProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "border-box" => BackgroundClipProperty::BorderBox,
            "padding-box" => BackgroundClipProperty::PaddingBox,
            "content-box" => BackgroundClipProperty::ContentBox,
            "initial" => BackgroundClipProperty::Initial,
            "inherit" => BackgroundClipProperty::Inherit,
            _ => return Err(format!("invalid background clip {:?}", s)),
        };
        Ok(p)
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

impl FromStr for BackgroundBlendModeProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
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
            _ => return Err(format!("invalid background blend mode {:?}", s)),
        };
        Ok(p)
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

impl FromStr for BackgroundAttachmentProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "scroll" => BackgroundAttachmentProperty::Scroll,
            "fixed" => BackgroundAttachmentProperty::Fixed,
            "local" => BackgroundAttachmentProperty::Local,
            "initial" => BackgroundAttachmentProperty::Initial,
            "inherit" => BackgroundAttachmentProperty::Inherit,
            _ => return Err(format!("invalid background attachment {:?}", s)),
        };
        Ok(p)
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

impl FromStr for PositionProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "static" => PositionProperty::Static,
            "relative" => PositionProperty::Relative,
            "absolute" => PositionProperty::Absolute,
            "fixed" => PositionProperty::Fixed,
            "sticky" => PositionProperty::Sticky,
            _ => return Err(format!("invalid position {:?}", s)),
        };
        Ok(p)
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

impl FromStr for FloatProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "left " => FloatProperty::Left,
            "right " => FloatProperty::Right,
            "none " => FloatProperty::None,
            "inline-start" => FloatProperty::InlineStart,
            "inline-end" => FloatProperty::InlineEnd,
            _ => return Err(format!("invalid float {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyValue<T> {
    Variable(String),
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
    AnimationDuration(PropertyValue<AnimationDirectionProperty>),
    AnimationFillMode(PropertyValue<AnimationFillModeProperty>),
    AnimationIterationCount(PropertyValue<usize>),
    AnimationName(PropertyValue<String>),
    AnimationPlayState(PropertyValue<AnimationPlayStateProperty>),
    AnimationTimingFunction(PropertyValue<AnimationTimingFunction>),
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
    ColumnCount(String),
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
    Variable(String, PropertyValue<String>),
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let res: Result<PropertyValue<()>, String> =
            CssParser::parse_prop_value_variable("var(--a)");
        let expected = Ok(PropertyValue::Variable("a".to_string()));
        assert_eq!(res, expected);
        let res: Result<PropertyValue<()>, String> =
            CssParser::parse_prop_value_variable("var(--foo)");
        let expected = Ok(PropertyValue::Variable("foo".to_string()));
        assert_eq!(res, expected);
        let res: Result<PropertyValue<()>, String> = CssParser::parse_prop_value_variable("--foo");
        let expected = Err(r#"given string is not a variable "--foo""#.to_string());
        assert_eq!(res, expected);
        let res: Result<PropertyValue<()>, String> = CssParser::parse_prop_value_variable("foo");
        let expected = Err(r#"given string is not a variable "foo""#.to_string());
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
    fn parse_animation_timing_function() {
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("linear").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Linear));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("ease").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Ease));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("ease-in").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::EaseIn));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("ease-out").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::EaseOut));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("ease-in-out").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::EaseInOut));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("step-start").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::StepStart));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("step-end").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::StepEnd));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(1,start)").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Steps(
            PropertyValue::Other(1),
            PropertyValue::Other(AnimationTimingFunctionSteps::Start),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(3,end)").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Steps(
            PropertyValue::Other(3),
            PropertyValue::Other(AnimationTimingFunctionSteps::End),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(0,start)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(-2,start)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"-2\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(0,end)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(-1,end)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"-1\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(end)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"end\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("steps(start)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"start\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("steps(0)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("steps(1)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("cubic-bezier(0.1,0.2,0.3,0.4)").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::CubicBezier(
            PropertyValue::Other(0.1),
            PropertyValue::Other(0.2),
            PropertyValue::Other(0.3),
            PropertyValue::Other(0.4),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("cubic-bezier(0.1, 0.2, 0.3, 0.4)").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::CubicBezier(
            PropertyValue::Other(0.1),
            PropertyValue::Other(0.2),
            PropertyValue::Other(0.3),
            PropertyValue::Other(0.4),
        )));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("cubic-bezier(0.1,0.2,0.3)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("cubic-bezier(0.1,0.2)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> =
            parse_prop_value("cubic-bezier(0.1)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("initial").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Initial));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunction> = parse_prop_value("inherit").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunction::Inherit));
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
            PropertyValue::Other(AnimationTimingFunction::Ease),
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
        let tokens = CssTokenizer::new(include_str!("../tests/a.css")).tokenize();
        let res = CssParser::new("../tests/a.css", tokens).parse();
        let expected: Result<Vec<Selector>, String> = Ok(vec![
            Selector {
                path: vec![SelectorPart::Class("foo".to_string())],
                block: Block { properties: vec![] },
            },
            Selector {
                path: vec![SelectorPart::Class("bar".to_string())],
                block: Block { properties: vec![] },
            },
            Selector {
                path: vec![SelectorPart::Class("foz".to_string())],
                block: Block { properties: vec![] },
            },
            Selector {
                path: vec![SelectorPart::Class("baz".to_string())],
                block: Block {
                    properties: vec![
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
                },
            },
        ]);
        assert_eq!(res, expected);
    }
}
