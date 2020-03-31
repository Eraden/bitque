use crate::shared::ToNode;
use crate::Msg;
use seed::{prelude::*, *};

pub struct StyledAvatar {
    pub avatar_url: Option<String>,
    pub size: u32,
    pub name: String,
    pub on_click: Option<EventHandler<Msg>>,
}

impl Default for StyledAvatar {
    fn default() -> Self {
        Self {
            avatar_url: None,
            size: 32,
            name: "".to_string(),
            on_click: None,
        }
    }
}

impl ToNode for StyledAvatar {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledAvatar) -> Node<Msg> {
    let StyledAvatar {
        avatar_url,
        size,
        name,
        on_click,
    } = values;
    let shared_style = format!("width: {size}px; height: {size}px", size = size);
    let handler = match on_click {
        None => vec![],
        Some(h) => vec![h],
    };
    match avatar_url {
        Some(url) => div![
            attrs![At::Class => "styledAvatar image", At::Style => format!("{shared}; background-image: url({url});", shared = shared_style, url = url)],
            handler,
        ],
        _ => div![
            attrs![At::Class => "styledAvatar letter", At::Style => shared_style],
            span![name],
            handler
        ],
    }
}
