use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

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

impl StyledAvatar {
    pub fn build() -> StyledAvatarBuilder {
        StyledAvatarBuilder {
            avatar_url: None,
            size: None,
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

pub struct StyledAvatarBuilder {
    pub avatar_url: Option<String>,
    pub size: Option<u32>,
    pub name: String,
    pub on_click: Option<EventHandler<Msg>>,
}

impl StyledAvatarBuilder {
    pub fn avatar_url<S>(mut self, avatar_url: S) -> Self
    where
        S: Into<String>,
    {
        self.avatar_url = Some(avatar_url.into());
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn on_click(mut self, on_click: EventHandler<Msg>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn build(self) -> StyledAvatar {
        StyledAvatar {
            avatar_url: self.avatar_url,
            size: self.size.unwrap_or(32),
            name: self.name,
            on_click: self.on_click,
        }
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
