use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug)]
pub struct StyledField {
    label: String,
    tip: Option<String>,
    input: Node<Msg>,
    class_list: Vec<String>,
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
    class_list: Vec<String>,
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

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn build(self) -> StyledField {
        StyledField {
            label: self.label.unwrap_or_default(),
            tip: self.tip,
            input: self.input.unwrap_or_else(|| empty![]),
            class_list: self.class_list,
        }
    }
}

pub fn render(values: StyledField) -> Node<Msg> {
    let StyledField {
        label,
        tip,
        input,
        mut class_list,
    } = values;
    let tip_node = match tip {
        Some(s) => div![attrs![At::Class => "styledTip"], s],
        _ => empty![],
    };
    class_list.push("styledField".to_string());

    div![
        attrs![At::Class => class_list.join(" ")],
        seed::label![attrs![At::Class => "styledLabel"], label],
        input,
        tip_node,
    ]
}
