use seed::prelude::*;
use seed::*;

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug, Clone, Default)]
pub struct StyledForm<'l> {
    pub heading: &'l str,
    pub fields: Vec<Node<Msg>>,
    pub on_submit: Option<EventHandler<Msg>>,
}

impl<'l> StyledForm<'l> {
    #[inline]
    pub fn render(self) -> Node<Msg> {
        let StyledForm {
            heading,
            fields,
            on_submit,
        } = self;
        let handlers = match on_submit {
            Some(handler) => vec![handler],
            _ => vec![],
        };
        seed::form![
            handlers,
            C!["styledForm"],
            div![C!["formElement"], div![C!["formHeading"], heading], fields],
        ]
    }
}

impl<'l> ToNode for StyledForm<'l> {
    #[inline]
    fn into_node(self) -> Node<Msg> {
        self.render()
    }
}
