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

impl<'l> ToNode for StyledForm<'l> {
    #[inline]
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[inline]
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
        C!["styledForm"],
        div![C!["formElement"], div![C!["formHeading"], heading], fields],
    ]
}
