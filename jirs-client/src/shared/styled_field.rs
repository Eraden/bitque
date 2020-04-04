use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug)]
pub struct StyledField {
    label: String,
    tip: Option<String>,
    input: Node<Msg>,
}

impl StyledField {
    pub fn build() -> StyledFieldBuilder {
        StyledFieldBuilder::default()
    }
}

impl ToNode for StyledField {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Default, Debug)]
pub struct StyledFieldBuilder {
    label: Option<String>,
    tip: Option<String>,
    input: Option<Node<Msg>>,
}

impl StyledFieldBuilder {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn tip<S>(mut self, tip: S) -> Self
    where
        S: Into<String>,
    {
        self.tip = Some(tip.into());
        self
    }

    pub fn input(mut self, input: Node<Msg>) -> Self {
        self.input = Some(input);
        self
    }

    pub fn build(self) -> StyledField {
        StyledField {
            label: self.label.unwrap_or_default(),
            tip: self.tip,
            input: self.input.unwrap_or_else(|| empty![]),
        }
    }
}

pub fn render(values: StyledField) -> Node<Msg> {
    let StyledField { label, tip, input } = values;
    let tip_node = match tip {
        Some(s) => div![attrs![At::Class => "styledTip"], s],
        _ => empty![],
    };
    div![
        attrs![At::Class => "styledField"],
        seed::label![attrs![At::Class => "styledLabel"], label],
        input,
        tip_node,
    ]
}
