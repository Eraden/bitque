use seed::prelude::*;
use seed::*;

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug)]
pub struct StyledAvatar<'l> {
    pub avatar_url: Option<&'l str>,
    pub size: u32,
    pub name: &'l str,
    pub on_click: Option<EventHandler<Msg>>,
    pub class_list: &'l str,
    pub user_index: usize,
}

impl<'l> StyledAvatar<'l> {
    pub fn render(self) -> Node<Msg> {
        let StyledAvatar {
            avatar_url,
            size,
            name,
            on_click,
            class_list,
            user_index,
        } = self;

        let index = user_index % 8;

        let shared_style = format!("width: {size}px; height: {size}px", size = size);
        let letter = name
            .chars()
            .rev()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_default();
        match avatar_url {
            Some(url) => {
                let style = format!(
                    "{shared}; background-image: url({url});",
                    shared = shared_style,
                    url = url
                );
                div![
                    C!["styledAvatar image", class_list],
                    attrs![At::Style => style, At::Title => name],
                    on_click
                ]
            }
            _ => {
                div![
                    C!["styledAvatar letter", class_list],
                    attrs![
                        At::Class => format!("avatarColor{}", index + 1),
                        At::Style => shared_style,
                        At::Title => name
                    ],
                    span![letter],
                    on_click,
                ]
            }
        }
    }
}

impl<'l> Default for StyledAvatar<'l> {
    fn default() -> Self {
        Self {
            avatar_url: None,
            size: 32,
            name: "",
            on_click: None,
            class_list: "",
            user_index: 0,
        }
    }
}

impl<'l> ToNode for StyledAvatar<'l> {
    #[inline(always)]
    fn into_node(self) -> Node<Msg> {
        self.render()
    }
}
