use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
pub enum Variant {
    Primary,
    Success,
    Danger,
    Secondary,
    Empty,
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        match self {
            Variant::Primary => "primary",
            Variant::Success => "success",
            Variant::Danger => "danger",
            Variant::Secondary => "secondary",
            Variant::Empty => "empty",
        }
        .to_string()
    }
}

#[derive(Default)]
pub struct StyledButtonBuilder {
    pub variant: Option<Variant>,
    pub disabled: Option<bool>,
    pub active: Option<bool>,
    pub text: Option<Option<String>>,
    pub icon: Option<Option<Node<Msg>>>,
    pub on_click: Option<Option<EventHandler<Msg>>>,
    pub children: Option<Vec<Node<Msg>>>,
}

impl StyledButtonBuilder {
    pub fn variant(mut self, value: Variant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn primary(self) -> Self {
        self.variant(Variant::Primary)
    }

    pub fn success(self) -> Self {
        self.variant(Variant::Success)
    }

    pub fn danger(self) -> Self {
        self.variant(Variant::Danger)
    }

    pub fn secondary(self) -> Self {
        self.variant(Variant::Secondary)
    }

    pub fn empty(self) -> Self {
        self.variant(Variant::Empty)
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(Some(value));
        self
    }

    pub fn icon<I>(mut self, value: I) -> Self
    where
        I: ToNode,
    {
        self.icon = Some(Some(value.into_node()));
        self
    }

    pub fn on_click(mut self, value: EventHandler<Msg>) -> Self {
        self.on_click = Some(Some(value));
        self
    }

    pub fn children(mut self, value: Vec<Node<Msg>>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn build(self) -> StyledButton {
        StyledButton {
            variant: self.variant.unwrap_or_else(|| Variant::Primary),
            disabled: self.disabled.unwrap_or_else(|| false),
            active: self.active.unwrap_or_else(|| false),
            text: self.text.unwrap_or_default(),
            icon: self.icon.unwrap_or_else(|| None),
            on_click: self.on_click.unwrap_or_else(|| None),
            children: self.children.unwrap_or_default(),
        }
    }
}

pub struct StyledButton {
    pub variant: Variant,
    pub disabled: bool,
    pub active: bool,
    pub text: Option<String>,
    pub icon: Option<Node<Msg>>,
    pub on_click: Option<EventHandler<Msg>>,
    pub children: Vec<Node<Msg>>,
}

impl StyledButton {
    pub fn build() -> StyledButtonBuilder {
        StyledButtonBuilder::default()
    }
}

impl ToNode for StyledButton {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledButton) -> Node<Msg> {
    let StyledButton {
        text,
        variant,
        disabled,
        active,
        icon,
        on_click,
        children,
    } = values;
    let mut class_list = vec!["styledButton".to_string(), variant.to_string()];
    if children.is_empty() && text.is_none() {
        class_list.push("iconOnly".to_string());
    }
    if active {
        class_list.push("isActive".to_string());
    }
    if icon.is_some() {
        class_list.push("withIcon".to_string());
    }
    let handler = match on_click {
        Some(h) if !disabled => vec![h],
        _ => vec![],
    };

    let icon_node = icon.unwrap_or_else(|| empty![]);
    let content = if children.is_empty() && text.is_none() {
        empty![]
    } else {
        span![
            attrs![At::Class => "text"],
            text.unwrap_or_default(),
            children
        ]
    };

    seed::button![
        attrs![
            At::Class => class_list.join(" "),
        ],
        handler,
        match disabled {
            true => vec![attrs![At::Disabled => true]],
            false => vec![],
        },
        icon_node,
        content,
    ]
}
