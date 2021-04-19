use std::str::FromStr;

use seed::prelude::*;
use seed::*;

use crate::Msg;

pub struct StyledLink<'l> {
    pub children: Vec<Node<Msg>>,
    pub class_list: &'l str,
    pub href: &'l str,
}

impl<'l> StyledLink<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledLink {
            children,
            class_list,
            href,
        } = self;

        let on_click = {
            let href = href.to_string();
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
            C!["styledLink", class_list],
            attrs![ At::Href => href ],
            on_click,
            children,
        ]
    }
}
