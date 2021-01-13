use {
    crate::{shared::ToNode, Msg},
    seed::{prelude::*, *},
};

pub struct StyledAvatar<'l> {
    avatar_url: Option<&'l str>,
    size: u32,
    name: &'l str,
    on_click: Option<EventHandler<Msg>>,
    class_list: Vec<&'l str>,
    user_index: usize,
}

impl<'l> Default for StyledAvatar<'l> {
    fn default() -> Self {
        Self {
            avatar_url: None,
            size: 32,
            name: "",
            on_click: None,
            class_list: vec![],
            user_index: 0,
        }
    }
}

impl<'l> StyledAvatar<'l> {
    #[inline(always)]
    pub fn build() -> StyledAvatarBuilder<'l> {
        StyledAvatarBuilder {
            avatar_url: None,
            size: None,
            name: "",
            on_click: None,
            class_list: vec![],
            user_index: 0,
        }
    }
}

impl<'l> ToNode for StyledAvatar<'l> {
    #[inline(always)]
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub struct StyledAvatarBuilder<'l> {
    avatar_url: Option<&'l str>,
    size: Option<u32>,
    name: &'l str,
    on_click: Option<EventHandler<Msg>>,
    class_list: Vec<&'l str>,
    user_index: usize,
}

impl<'l> StyledAvatarBuilder<'l> {
    #[inline(always)]
    pub fn avatar_url<'m: 'l>(mut self, avatar_url: &'m str) -> Self {
        if !avatar_url.is_empty() {
            self.avatar_url = Some(avatar_url);
        }
        self
    }

    #[inline(always)]
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    #[inline(always)]
    pub fn name<'m: 'l>(mut self, name: &'m str) -> Self {
        self.name = name;
        self
    }

    #[inline(always)]
    pub fn on_click(mut self, on_click: EventHandler<Msg>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    #[inline(always)]
    pub fn add_class<'m: 'l>(mut self, name: &'m str) -> Self {
        self.class_list.push(name);
        self
    }

    #[inline(always)]
    pub fn user_index(mut self, user_index: usize) -> Self {
        self.user_index = user_index;
        self
    }

    #[inline(always)]
    pub fn build(self) -> StyledAvatar<'l> {
        StyledAvatar {
            avatar_url: self.avatar_url,
            size: self.size.unwrap_or(32),
            name: self.name,
            on_click: self.on_click,
            class_list: self.class_list,
            user_index: self.user_index,
        }
    }
}

pub fn render(values: StyledAvatar) -> Node<Msg> {
    let StyledAvatar {
        avatar_url,
        size,
        name,
        on_click,
        class_list,
        user_index,
    } = values;

    let index = user_index % 8;

    let shared_style = format!("width: {size}px; height: {size}px", size = size);
    let class_list: Attrs = {
        let s: String = class_list.join(" ");
        C![s.as_str()]
    };
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
                C!["styledAvatar image"],
                class_list,
                attrs![At::Style => style, At::Title => name],
                on_click
            ]
        }
        _ => {
            div![
                C!["styledAvatar letter"],
                class_list,
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
