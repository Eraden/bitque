use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug, Clone)]
pub struct StyledForm {
    heading: String,
    fields: Vec<Node<Msg>>,
    on_submit: Option<EventHandler<Msg>>,
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
    on_submit: Option<EventHandler<Msg>>,
}

impl StyledFormBuilder {
    pub fn add_field(mut self, node: Node<Msg>) -> Self {
        self.fields.push(node);
        self
    }

    pub fn try_field(mut self, node: Option<Node<Msg>>) -> Self {
        if let Some(n) = node {
            self.fields.push(n);
        }
        self
    }

    pub fn heading<S>(mut self, heading: S) -> Self
    where
        S: Into<String>,
    {
        self.heading = heading.into();
        self
    }

    pub fn on_submit(mut self, on_submit: EventHandler<Msg>) -> Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn build(self) -> StyledForm {
        StyledForm {
            heading: self.heading,
            fields: self.fields,
            on_submit: self.on_submit,
        }
    }
}

pub fn render(values: StyledForm) -> Node<Msg> {
    let StyledForm {
        heading,
        fields,
        on_submit,
    } = values;
    let handlers = match on_submit {
        Some(handler) => vec![handler],
        _ => vec![],
    };
    seed::form![
        handlers,
        attrs![At::Class => "styledForm"],
        div![
            class!["formElement"],
            div![class!["formHeading"], heading],
            fields
        ],
    ]
}
