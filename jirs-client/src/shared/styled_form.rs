use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug, Clone)]
pub struct StyledForm<'l> {
    heading: &'l str,
    fields: Vec<Node<Msg>>,
    on_submit: Option<EventHandler<Msg>>,
}

impl<'l> StyledForm<'l> {
    pub fn build() -> StyledFormBuilder<'l> {
        StyledFormBuilder::default()
    }
}

impl<'l> ToNode for StyledForm<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Debug, Default)]
pub struct StyledFormBuilder<'l> {
    fields: Vec<Node<Msg>>,
    heading: &'l str,
    on_submit: Option<EventHandler<Msg>>,
}

impl<'l> StyledFormBuilder<'l> {
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

    pub fn heading(mut self, heading: &'l str) -> Self {
        self.heading = heading;
        self
    }

    pub fn on_submit(mut self, on_submit: EventHandler<Msg>) -> Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn build(self) -> StyledForm<'l> {
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
        div![C!["formElement"], div![C!["formHeading"], heading], fields],
    ]
}
