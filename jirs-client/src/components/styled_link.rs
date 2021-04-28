use std::str::FromStr;

use seed::prelude::*;
use seed::*;

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
        let on_click = {
            let href = self.href.to_string();
            mouse_ev("click", move |ev| {
                if href.starts_with('/') {
                    ev.prevent_default();
                    ev.stop_propagation();
                    if let Ok(url) = seed::Url::from_str(href.as_str()) {
                        url.go_and_push();
                    }
                }

                None as Option<Msg>
            })
        };

        a![
            C!["styledLink", self.class_list],
            attrs![ At::Href => self.href ],
            on_click,
            self.children,
        ]
    }
}
