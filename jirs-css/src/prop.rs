use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::vec::IntoIter;

use crate::colors::Color;
use crate::colors::{parse_hsla, parse_rgba};

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
                '{' | '}' | ';' | ':' => {
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
        f.write_str(format!("({}:{}:{})", self.line, self.line_character, self.character).as_str())
    }
}

#[derive(Debug)]
pub struct CssParser {
    it: Peekable<IntoIter<String>>,
    selectors: Vec<Selector>,
    pos: ParserPosition,
}

impl CssParser {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            it: tokens.into_iter().peekable(),
            selectors: vec![],
            pos: ParserPosition {
                line: 0,
                line_character: 0,
                character: 0,
            },
        }
    }

    pub fn parse(mut self) -> Result<Vec<Selector>, String> {
        while let Some(token) = self.it.next() {
            let selector = self.parse_selector(token.as_str())?;
            self.selectors.push(selector);
        }
        Ok(self.selectors)
    }

    fn parse_selector(&mut self, token: &str) -> Result<Selector, String> {
        let mut path = vec![];

        if let Ok(part) = token.parse::<SelectorPart>() {
            path.push(part);
            self.parse_selector_path(&mut path);
        }
        let block = self
            .expect_consume()
            .and_then(|s| self.parse_block(s.as_str()))?;

        Ok(Selector { path, block })
    }

    fn parse_selector_path(&mut self, path: &mut Vec<SelectorPart>) {
        self.skip_white();
        while let Some(token) = self.it.peek() {
            if token.as_str() == "{" {
                break;
            }
            match self.consume().unwrap_or_default().parse::<SelectorPart>() {
                Ok(part) => {
                    path.push(part);
                }
                _ => break,
            };
            self.skip_white();
        }
    }

    fn parse_block(&mut self, token: &str) -> Result<Block, String> {
        if token != "{" {
            return Err(format!("expect to find block but found {:?}", token));
        }
        let mut block = Block { properties: vec![] };
        self.skip_white();
        while let Some(token) = self.consume() {
            if token.as_str() == "}" {
                self.consume();
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
            "align-content" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected_prop_value::<AlignContentProperty>()?;
                self.consume_expected(";")?;
                Property::AlignContent(p)
            }
            "align-items" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected_prop_value::<AlignItemsProperty>()?;
                self.consume_expected(";")?;
                Property::AlignItems(p)
            }
            "align-self" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected_prop_value::<AlignSelfProperty>()?;
                self.consume_expected(";")?;
                Property::AlignSelf(p)
            }
            "all" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected_prop_value::<AllProperty>()?;
                self.consume_expected(";")?;
                Property::All(p)
            }
            "animation" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let def = self.expect_consume()?;
                let p = match def.as_str() {
                    "initial" => PropertyValue::Other(AnimationProperty::Initial),
                    "inherit" => PropertyValue::Other(AnimationProperty::Inherit),
                    _ if def.starts_with("--") => PropertyValue::Variable(def[2..].to_string()),
                    _ => {
                        let name = def;
                        self.skip_white();

                        let duration = if self.current_is_semicolon() {
                            PropertyValue::Other(TimeProperty::Seconds(0))
                        } else {
                            let v = self.parse_expected_prop_value::<TimeProperty>()?;
                            self.skip_white();
                            v
                        };
                        let timing = if self.current_is_semicolon() {
                            PropertyValue::Other(AnimationTimingFunction::Ease)
                        } else {
                            let v = self.parse_expected_prop_value::<AnimationTimingFunction>()?;
                            self.skip_white();
                            v
                        };
                        let delay = if self.current_is_semicolon() {
                            PropertyValue::Other(AnimationDelayProperty::Time(
                                TimeProperty::Seconds(0),
                            ))
                        } else {
                            let v = self.parse_expected_prop_value::<AnimationDelayProperty>()?;
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
                            let v =
                                self.parse_expected_prop_value::<AnimationDirectionProperty>()?;
                            self.skip_white();
                            v
                        };
                        let fill_mode = if self.current_is_semicolon() {
                            PropertyValue::Other(AnimationFillModeProperty::None)
                        } else {
                            let v =
                                self.parse_expected_prop_value::<AnimationFillModeProperty>()?;
                            self.skip_white();
                            v
                        };
                        let play_state = if self.current_is_semicolon() {
                            PropertyValue::Other(AnimationPlayStateProperty::Running)
                        } else {
                            let v =
                                self.parse_expected_prop_value::<AnimationPlayStateProperty>()?;
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
                Property::Animation(p)
            }
            "animation-delay" => {
                let d = self.parse_expected_prop_value::<TimeProperty>()?;
                self.consume_expected(";")?;
                Property::AnimationDelay(d)
            }
            "animation-direction" => {
                let p = self.parse_expected_prop_value::<AnimationDirectionProperty>()?;
                self.consume_expected(";")?;
                Property::AnimationDirection(p)
            }
            "animation-duration" => {
                let p = self.parse_expected_prop_value::<AnimationDirectionProperty>()?;
                self.consume_expected(";")?;
                Property::AnimationDuration(p)
            }
            "animation-fill-mode" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::AnimationFillMode(p)
            }
            "animation-iteration-count" => {
                let count = self.expect_consume()?;
                let p = if count.starts_with("--") {
                    PropertyValue::Variable(count[2..].to_string())
                } else {
                    PropertyValue::Other(count.parse::<usize>().map_err(|_| {
                        format!("invalid iteration count, expect number got {:?}", count)
                    })?)
                };

                self.consume_expected(";")?;
                Property::AnimationIterationCount(p)
            }
            "animation-name" => {
                self.consume_expected(";")?;
                let name = match self.expect_consume()?.as_str() {
                    name @ _ if name.starts_with("--") => {
                        PropertyValue::Variable(name[2..].to_string())
                    }
                    name @ _ => PropertyValue::Other(name.to_string()),
                };
                Property::AnimationName(name)
            }
            "animation-play-state" => {
                let p = self.parse_expected_prop_value()?;
                self.consume_expected(";")?;
                Property::AnimationPlayState(p)
            }
            "animation-timing-function" => {
                let p = self.parse_expected_prop_value()?;
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
                let p = self.parse_expected_prop_value()?;
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
            //     "clear" => Property::Clear,
            //     "clip" => Property::Clip,
            //     "clip-path" => Property::ClipPath,
            //     "color" => Property::Color,
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
            "display" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected::<DisplayProperty>()?;
                self.consume_expected(";")?;
                Property::Display(p)
            }
            //     "empty-cells" => Property::EmptyCells,
            //     "filter" => Property::Filter,
            //     "flex" => Property::Flex,
            //     "flex-basis" => Property::FlexBasis,
            //     "flex-direction" => Property::FlexDirection,
            //     "flex-flow" => Property::FlexFlow,
            //     "flex-grow" => Property::FlexGrow,
            //     "flex-shrink" => Property::FlexShrink,
            //     "flex-wrap" => Property::FlexWrap,
            //     "float" => Property::Float,
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
            "justify-content" => {
                self.skip_white();
                self.consume_expected(":")?;
                self.skip_white();
                let p = self.parse_expected_prop_value::<JustifyContentProperty>()?;
                self.consume_expected(";")?;
                Property::JustifyContent(p)
            }
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
            //     "position" => Property::Position,
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
                    self.pos.line_character += 0;
                }
                _ => {
                    self.pos.character += s.len();
                    self.pos.line_character += s.len();
                }
            }
        }
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
            .ok_or_else(|| "expect to have parsable token but nothing was found".to_string())
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

    fn parse_expected_prop_value<ExpectedType>(
        &mut self,
    ) -> Result<PropertyValue<ExpectedType>, String>
    where
        ExpectedType: FromStr<Err = String>,
    {
        let s = self.expect_consume()?;
        if s.starts_with("--") {
            Ok(PropertyValue::Variable(s[2..].to_string()))
        } else {
            s.parse::<ExpectedType>().map(|v| PropertyValue::Other(v))
        }
    }

    fn current_is_semicolon(&mut self) -> bool {
        self.peek().map(|s| s.as_str() == ";").unwrap_or_default()
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

impl FromStr for TimeProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s {
            "initial" => TimeProperty::Initial,
            "inherit" => TimeProperty::Inherit,
            _ if s.ends_with("ms") => match s[0..(s.len() - 2)].parse::<i32>() {
                Ok(n) => TimeProperty::MilliSeconds(n),
                _ => return Err("invalid time".to_string()),
            },
            _ if s.ends_with("s") => match s[0..(s.len() - 1)].parse::<i32>() {
                Ok(n) => TimeProperty::Seconds(n),
                _ => return Err("invalid time".to_string()),
            },
            _ => return Err("invalid time".to_string()),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimingFunctionSteps {
    Start,
    End,
}

impl FromStr for AnimationTimingFunctionSteps {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "start" => Ok(AnimationTimingFunctionSteps::Start),
            "end" => Ok(AnimationTimingFunctionSteps::End),
            _ => Err(format!("invalid animation timing function step {:?}", s)),
        }
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
    Steps(u32, AnimationTimingFunctionSteps),
    CubicBezier(f64, f64, f64, f64),
    Initial,
    Inherit,
}

impl FromStr for AnimationTimingFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s.to_string().as_str() {
            "linear" => AnimationTimingFunction::Linear,
            "ease" => AnimationTimingFunction::Ease,
            "ease-in" => AnimationTimingFunction::EaseIn,
            "ease-out" => AnimationTimingFunction::EaseOut,
            "ease-in-out" => AnimationTimingFunction::EaseInOut,
            "step-start" => AnimationTimingFunction::StepStart,
            "step-end" => AnimationTimingFunction::StepEnd,
            "initial" => AnimationTimingFunction::Initial,
            "inherit" => AnimationTimingFunction::Inherit,
            _ if s.starts_with("steps(") => {
                let n_start = "steps(".len();
                let (n_end, _) = s
                    .char_indices()
                    .find(|(_idx, c)| *c == ',')
                    .ok_or_else(|| format!("invalid animation timing function {:?}", s))?;
                let b = s[n_start..n_end]
                    .trim()
                    .parse::<u32>()
                    .map_err(|_| format!("invalid animation timing function {:?}", s))?;
                if b == 0 {
                    return Err(format!("invalid animation timing function {:?}", s));
                }
                let c = s[(n_end + 1)..(s.len() - 1)]
                    .trim()
                    .parse::<AnimationTimingFunctionSteps>()
                    .map_err(|_| format!("invalid animation timing function {:?}", s))?;
                AnimationTimingFunction::Steps(b, c)
            }
            _ if s.starts_with("cubic-bezier(") => {
                let v: Vec<f64> = s["cubic-bezier(".len()..(s.len() - 1)]
                    .split(',')
                    .filter_map(|p| p.trim().parse::<f64>().ok())
                    .collect();
                if v.len() != 4 {
                    return Err(format!("invalid animation timing function {:?}", s));
                }
                AnimationTimingFunction::CubicBezier(v[0], v[1], v[2], v[3])
            }
            // "steps(int,start|end)" => AnimationTimingFunction::Steps(int, start | end),
            // "cubic-bezier(n,n,n,n)" => AnimationTimingFunction::CubicBezier(n, n, n, n),
            _ => return Err(format!("invalid animation timing function {:?}", s)),
        };
        Ok(p)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationDelayProperty {
    Time(TimeProperty),
    Initial,
    Inherit,
}

impl FromStr for AnimationDelayProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "initial" => Ok(AnimationDelayProperty::Initial),
            "inherit" => Ok(AnimationDelayProperty::Inherit),
            _ => s
                .parse::<TimeProperty>()
                .map(|p| AnimationDelayProperty::Time(p)),
        }
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
pub enum ColorProperty {
    Name(String),
    Rgba(u8, u8, u8, u8),
    Hsla(u16, u8, u8, f64),
    Current,
}

impl FromStr for ColorProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s.trim() {
            "currentColor" => ColorProperty::Current,
            _ if s.len() == 7 && s.starts_with('#') => {
                let r = u8::from_str_radix(&s[1..=2], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[3..=4], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[5..=6], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(r, g, b, 255)
            }
            _ if s.len() == 4 && s.starts_with('#') => {
                let _x = &s[1..=1];
                let r = u8::from_str_radix(&s[1..=1].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[2..=2].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[3..=3].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(r, g, b, 255)
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
                ColorProperty::Rgba(r, g, b, a)
            }
            _ if s.trim().to_lowercase().starts_with("rgba(") => {
                let (r, g, b, a) = parse_rgba(s.trim(), true)?;
                ColorProperty::Rgba(r, g, b, a)
            }
            _ if s.trim().to_lowercase().starts_with("rgb(") => {
                let (r, g, b, a) = parse_rgba(s.trim(), false)?;
                ColorProperty::Rgba(r, g, b, a)
            }
            _ if s.trim().to_lowercase().starts_with("hsla(") => {
                let (h, s, l, a) = parse_hsla(s.trim(), true)?;
                ColorProperty::Hsla(h, s, l, a)
            }
            _ if s.trim().to_lowercase().starts_with("hsl(") => {
                let (h, s, l, a) = parse_hsla(s.trim(), false)?;
                ColorProperty::Hsla(h, s, l, a)
            }

            _ => s
                .parse::<Color>()
                .map(|c| c.to_hex())
                .and_then(|s| s.parse::<ColorProperty>())?,
        };
        Ok(p)
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
    AnimationDelay(PropertyValue<TimeProperty>),
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
    Clear(String),
    Clip(String),
    ClipPath(String),
    Color(String),
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
    Display(DisplayProperty),
    EmptyCells(String),
    Filter(String),
    Flex(String),
    FlexBasis(String),
    FlexDirection(String),
    FlexFlow(String),
    FlexGrow(String),
    FlexShrink(String),
    FlexWrap(String),
    Float(String),
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
    Position(String),
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
    Variable(String, String),
}

#[cfg(test)]
mod tests {
    use crate::prop::*;

    #[test]
    fn parse_color() {
        let res = "#123".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(17, 34, 51, 255));
        assert_eq!(res, expected);
        let res = "#010203".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(1, 2, 3, 255));
        assert_eq!(res, expected);
        let res = "#fff".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(255, 255, 255, 255));
        assert_eq!(res, expected);
        let res = "#ffffff".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(255, 255, 255, 255));
        assert_eq!(res, expected);
        let res = "#abcdef".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(171, 205, 239, 255));
        assert_eq!(res, expected);
        let res = "currentColor".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Current);
        assert_eq!(res, expected);
        let res = "rgb(1,2,3)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(1, 2, 3, 255));
        assert_eq!(res, expected);
        let res = "rgb(1,2,3,.2)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(1, 2, 3, 255));
        assert_eq!(res, expected);
        let res = "rgba(1,2,3,.1)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(1, 2, 3, 25));
        assert_eq!(res, expected);
        let res = "hsla(100,20%,30%,.4)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Hsla(100, 20, 30, 0.4f64));
        assert_eq!(res, expected);
        let res = "hsl(100,20%,30%)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Hsla(100, 20, 30, 1.0f64));
        assert_eq!(res, expected);
        let res = "hsl(100,20%,30%,0.5)".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Hsla(100, 20, 30, 1.0f64));
        assert_eq!(res, expected);
        let res = "CornflowerBlue".parse::<ColorProperty>();
        let expected = Ok(ColorProperty::Rgba(100, 149, 237, 255));
        assert_eq!(res, expected);
        let res = "foo".parse::<ColorProperty>();
        let expected = Err("invalid predefined color \"foo\"".to_string());
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
        let res = "".parse::<TimeProperty>();
        let expected = Err("invalid time".to_string());
        assert_eq!(res, expected);
        let res = "as".parse::<TimeProperty>();
        let expected = Err("invalid time".to_string());
        assert_eq!(res, expected);
        let res = "3s".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::Seconds(3));
        assert_eq!(res, expected);
        let res = "2s".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::Seconds(2));
        assert_eq!(res, expected);
        let res = "-5s".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::Seconds(-5));
        assert_eq!(res, expected);
        let res = "3ms".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::MilliSeconds(3));
        assert_eq!(res, expected);
        let res = "2ms".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::MilliSeconds(2));
        assert_eq!(res, expected);
        let res = "-5ms".parse::<TimeProperty>();
        let expected = Ok(TimeProperty::MilliSeconds(-5));
        assert_eq!(res, expected);
    }

    #[test]
    fn parse_animation_timing_function() {
        let res = "linear".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Linear);
        assert_eq!(res, expected);
        let res = "ease".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Ease);
        assert_eq!(res, expected);
        let res = "ease-in".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::EaseIn);
        assert_eq!(res, expected);
        let res = "ease-out".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::EaseOut);
        assert_eq!(res, expected);
        let res = "ease-in-out".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::EaseInOut);
        assert_eq!(res, expected);
        let res = "step-start".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::StepStart);
        assert_eq!(res, expected);
        let res = "step-end".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::StepEnd);
        assert_eq!(res, expected);
        let res = "steps(1,start)".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Steps(
            1,
            AnimationTimingFunctionSteps::Start,
        ));
        assert_eq!(res, expected);
        let res = "steps(3,end)".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Steps(
            3,
            AnimationTimingFunctionSteps::End,
        ));
        assert_eq!(res, expected);
        let res = "steps(0,start)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(0,start)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(-2,start)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(-2,start)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(0,end)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(0,end)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(-1,end)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(-1,end)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(end)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(end)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(start)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(start)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(0)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(0)\"".to_string());
        assert_eq!(res, expected);
        let res = "steps(1)".parse::<AnimationTimingFunction>();
        let expected = Err("invalid animation timing function \"steps(1)\"".to_string());
        assert_eq!(res, expected);
        let res = "cubic-bezier(0.1,0.2,0.3,0.4)".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::CubicBezier(0.1, 0.2, 0.3, 0.4));
        assert_eq!(res, expected);
        let res = "cubic-bezier(0.1, 0.2, 0.3, 0.4)".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::CubicBezier(0.1, 0.2, 0.3, 0.4));
        assert_eq!(res, expected);
        let res = "cubic-bezier(0.1,0.2,0.3)".parse::<AnimationTimingFunction>();
        let expected =
            Err(r#"invalid animation timing function "cubic-bezier(0.1,0.2,0.3)""#.to_string());
        assert_eq!(res, expected);
        let res = "cubic-bezier(0.1,0.2)".parse::<AnimationTimingFunction>();
        let expected =
            Err(r#"invalid animation timing function "cubic-bezier(0.1,0.2)""#.to_string());
        assert_eq!(res, expected);
        let res = "cubic-bezier(0.1)".parse::<AnimationTimingFunction>();
        let expected = Err(r#"invalid animation timing function "cubic-bezier(0.1)""#.to_string());
        assert_eq!(res, expected);
        let res = "initial".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Initial);
        assert_eq!(res, expected);
        let res = "inherit".parse::<AnimationTimingFunction>();
        let expected = Ok(AnimationTimingFunction::Inherit);
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
    fn parse_empty_p_selector() {
        let source = r#"p{}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string())],
                block: Block {
                    properties: vec![Property::Display(DisplayProperty::Block),]
                },
            }]),
        )
    }

    #[test]
    fn parse_long_path() {
        let source = r#"p p {}"#;
        let tokens = CssTokenizer::new(source).tokenize();
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
        assert_eq!(
            result,
            Ok(vec![Selector {
                path: vec![SelectorPart::TagName("p".to_string()),],
                block: Block {
                    properties: vec![
                        Property::Display(DisplayProperty::Block),
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
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
        let result = CssParser::new(tokens).parse();
        let animation = Property::Animation(PropertyValue::Other(AnimationProperty::Custom(
            "some".to_string(),
            PropertyValue::Other(TimeProperty::Seconds(0)),
            PropertyValue::Other(AnimationTimingFunction::Ease),
            PropertyValue::Other(AnimationDelayProperty::Time(TimeProperty::Seconds(0))),
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
}
