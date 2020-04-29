use crate::prop::{CssParser, ParseToken, Parser, PropertyValue, TimeProperty, Token, ValueResult};

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
        let p = match self.expect_consume()?.as_str() {
            "normal" => AnimationDirectionProperty::Normal,
            "reverse" => AnimationDirectionProperty::Reverse,
            "alternate" => AnimationDirectionProperty::Alternate,
            "alternate-reverse" => AnimationDirectionProperty::AlternateReverse,
            "initial" => AnimationDirectionProperty::Initial,
            "inherit" => AnimationDirectionProperty::Inherit,
            _ => return Err(format!("invalid animation direction {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
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

impl Token for AnimationFillModeProperty {}

impl ParseToken<AnimationFillModeProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<AnimationFillModeProperty> {
        let p = match self.expect_consume()?.as_str() {
            "none" => AnimationFillModeProperty::None,
            "forwards" => AnimationFillModeProperty::Forwards,
            "backwards" => AnimationFillModeProperty::Backwards,
            "both" => AnimationFillModeProperty::Both,
            "initial" => AnimationFillModeProperty::Initial,
            "inherit" => AnimationFillModeProperty::Inherit,
            _ => return Err(format!("invalid animation fill mode {:?}", self.current)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationPlayStateProperty {
    Paused,
    Running,
    Initial,
    Inherit,
}

impl Token for AnimationPlayStateProperty {}

impl ParseToken<AnimationPlayStateProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<AnimationPlayStateProperty> {
        self.skip_white();
        let name = self.expect_consume()?;
        let p = match name.as_str() {
            "paused" => AnimationPlayStateProperty::Paused,
            "running" => AnimationPlayStateProperty::Running,
            "initial" => AnimationPlayStateProperty::Initial,
            "inherit" => AnimationPlayStateProperty::Inherit,
            _ => return Err(format!("invalid animation play state {:?}", name)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimingFunctionStepsProperty {
    Start,
    End,
}

impl Token for AnimationTimingFunctionStepsProperty {}

impl ParseToken<AnimationTimingFunctionStepsProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<AnimationTimingFunctionStepsProperty> {
        let s = self.expect_consume()?;
        let p = match s.to_lowercase().as_str() {
            "start" => AnimationTimingFunctionStepsProperty::Start,
            "end" => AnimationTimingFunctionStepsProperty::End,
            _ => return Err(format!("invalid animation timing function step {:?}", s)),
        };
        Ok(PropertyValue::Other(p))
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimingFunctionProperty {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    Steps(
        PropertyValue<u32>,
        PropertyValue<AnimationTimingFunctionStepsProperty>,
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

impl Token for AnimationTimingFunctionProperty {}

impl ParseToken<AnimationTimingFunctionProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<AnimationTimingFunctionProperty> {
        let current = self.expect_consume()?;
        let p = match current.as_str() {
            "linear" => AnimationTimingFunctionProperty::Linear,
            "ease" => AnimationTimingFunctionProperty::Ease,
            "ease-in" => AnimationTimingFunctionProperty::EaseIn,
            "ease-out" => AnimationTimingFunctionProperty::EaseOut,
            "ease-in-out" => AnimationTimingFunctionProperty::EaseInOut,
            "step-start" => AnimationTimingFunctionProperty::StepStart,
            "step-end" => AnimationTimingFunctionProperty::StepEnd,
            "initial" => AnimationTimingFunctionProperty::Initial,
            "inherit" => AnimationTimingFunctionProperty::Inherit,
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
                self.consume_semicolon()?;
                AnimationTimingFunctionProperty::Steps(b, c)
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
                self.consume_semicolon()?;
                AnimationTimingFunctionProperty::CubicBezier(a, b, c, d)
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
        let p = match self.peek().cloned().unwrap_or_default().as_str() {
            "initial" => {
                self.expect_consume()?;
                AnimationDelayProperty::Initial
            }
            "inherit" => {
                self.expect_consume()?;
                AnimationDelayProperty::Inherit
            }
            _ => AnimationDelayProperty::Time(self.parse_token()?),
        };
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
        PropertyValue<AnimationTimingFunctionProperty>,
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
        eprintln!("only full animation is supported!");
        if let Some(v) = self.try_parse_variable() {
            return Ok(PropertyValue::Variable(v));
        }
        let def = self
            .peek()
            .cloned()
            .ok_or_else(|| "expect to find token but EOF".to_string())?;
        let p = match def.as_str() {
            "initial" => {
                self.expect_consume()?;
                PropertyValue::Other(AnimationProperty::Initial)
            }
            "inherit" => {
                self.expect_consume()?;
                PropertyValue::Other(AnimationProperty::Inherit)
            }
            _ => {
                let duration = if self.next_is_semicolon() {
                    PropertyValue::Other(TimeProperty::Seconds(0))
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let timing = if self.next_is_semicolon() {
                    PropertyValue::Other(AnimationTimingFunctionProperty::Ease)
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let delay = if self.next_is_semicolon() {
                    PropertyValue::Other(AnimationDelayProperty::Time(PropertyValue::Other(
                        TimeProperty::Seconds(0),
                    )))
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let iteration_count = if self.next_is_semicolon() {
                    PropertyValue::Other(1)
                } else {
                    let v = self.expect_consume()?.parse::<usize>().map_err(|_| {
                        format!(
                            "invalid animation iteration count, expect number got {:?}",
                            self.current
                        )
                    })?;
                    self.skip_white();
                    PropertyValue::Other(v)
                };
                let direction = if self.next_is_semicolon() {
                    PropertyValue::Other(AnimationDirectionProperty::Normal)
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let fill_mode = if self.next_is_semicolon() {
                    PropertyValue::Other(AnimationFillModeProperty::None)
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let play_state = if self.next_is_semicolon() {
                    PropertyValue::Other(AnimationPlayStateProperty::Running)
                } else {
                    let v = self.parse_token()?;
                    self.skip_white();
                    v
                };
                let name = self.expect_consume()?;

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
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::prop::CssTokenizer;

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

    #[test]
    fn parse_animation_timing_function() {
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("linear").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::Linear,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("ease").parse_token();
        let expected = Ok(PropertyValue::Other(AnimationTimingFunctionProperty::Ease));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("ease-in").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::EaseIn,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("ease-out").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::EaseOut,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("ease-in-out").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::EaseInOut,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("step-start").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::StepStart,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("step-end").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::StepEnd,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(1,start)").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::Steps(
                PropertyValue::Other(1),
                PropertyValue::Other(AnimationTimingFunctionStepsProperty::Start),
            ),
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(3,end)").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::Steps(
                PropertyValue::Other(3),
                PropertyValue::Other(AnimationTimingFunctionStepsProperty::End),
            ),
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(0,start)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(-2,start)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"-2\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(0,end)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(-1,end)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"-1\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(end)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"end\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(start)").parse_token();
        let expected =
            Err("invalid token, expect number greater or equal 0 got \"start\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(0)").parse_token();
        let expected = Err(
            "invalid animation timing function, number of iterations must be greater than 0"
                .to_string(),
        );
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("steps(1)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("cubic-bezier(0.1,0.2,0.3,0.4)").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::CubicBezier(
                PropertyValue::Other(0.1),
                PropertyValue::Other(0.2),
                PropertyValue::Other(0.3),
                PropertyValue::Other(0.4),
            ),
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("cubic-bezier(0.1, 0.2, 0.3, 0.4)").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::CubicBezier(
                PropertyValue::Other(0.1),
                PropertyValue::Other(0.2),
                PropertyValue::Other(0.3),
                PropertyValue::Other(0.4),
            ),
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("cubic-bezier(0.1,0.2,0.3)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("cubic-bezier(0.1,0.2)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("cubic-bezier(0.1)").parse_token();
        let expected = Err("expect to find token \",\" but found \")\"".to_string());
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("initial").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::Initial,
        ));
        assert_eq!(res, expected);
        let res: ValueResult<AnimationTimingFunctionProperty> =
            parse_prop_value("inherit").parse_token();
        let expected = Ok(PropertyValue::Other(
            AnimationTimingFunctionProperty::Inherit,
        ));
        assert_eq!(res, expected);
    }
}
