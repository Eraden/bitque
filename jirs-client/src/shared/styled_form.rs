use crate::shared::ToNode;
use crate::Msg;
use seed::{prelude::*, *};

#[derive(Debug, Clone)]
pub struct StyledForm {
    heading: String,
    fields: Vec<Node<Msg>>,
}

impl StyledForm {
    pub fn build() -> StyledFormBuilder {
        StyledFormBuilder::default()
    }
}

impl ToNode for StyledForm {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Debug, Default)]
pub struct StyledFormBuilder {
    fields: Vec<Node<Msg>>,
    heading: String,
}

impl StyledFormBuilder {
    pub fn add_field(mut self, node: Node<Msg>) -> Self {
        self.fields.push(node);
        self
    }

    pub fn heading<S>(mut self, heading: S) -> Self
    where
        S: Into<String>,
    {
        self.heading = heading.into();
        self
    }

    pub fn build(self) -> StyledForm {
        StyledForm {
            heading: self.heading,
            fields: self.fields,
        }
    }
}

pub fn render(values: StyledForm) -> Node<Msg> {
    let StyledForm { heading, fields } = values;
    div![
        attrs![At::Class => "styledForm"],
        div![
            attrs![At::Class => "formElement"],
            div![attrs![At::Class => "heading"], heading],
            fields
        ],
    ]
}
