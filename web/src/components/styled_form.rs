use seed::prelude::*;
use seed::*;

use crate::Msg;

#[derive(Debug, Clone, Default)]
pub struct StyledForm<'l> {
    pub heading: &'l str,
    pub fields: Vec<Node<Msg>>,
    pub on_submit: Option<EventHandler<Msg>>,
}

impl<'l> StyledForm<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        seed::form![
            self.on_submit,
            C!["styledForm"],
            div![
                C!["formElement"],
                div![C!["formHeading"], self.heading],
                self.fields
            ],
        ]
    }
}
