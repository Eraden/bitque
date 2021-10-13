use seed::*;
use seed::prelude::*;

use crate::Msg;

#[derive(Debug, Default)]
pub struct StyledLink<'l> {
    pub children: Vec<Node<Msg>>,
    pub class_list: &'l str,
    pub href: &'l str,
    pub disabled: bool,
}

impl<'l> StyledLink<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let on_click = super::events::on_click_change_page(self.href.to_string());

        a![
            C!["styledLink", self.class_list],
            attrs![ At::Href => self.href ],
            on_click,
            self.children,
        ]
    }
}
