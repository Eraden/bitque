use seed::{prelude::*, *};

use crate::model::Icon;
use crate::shared::styled_icon;
use crate::Msg;

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

pub struct StyledButton {
    pub variant: Variant,
    pub icon_only: bool,
    pub disabled: bool,
    pub active: bool,
    pub text: Option<String>,
    pub icon: Option<Icon>,
}

impl Into<Node<Msg>> for StyledButton {
    fn into(self) -> Node<Msg> {
        styled_button(self)
    }
}

impl StyledButton {
    pub fn into_node(self) -> Node<Msg> {
        self.into()
    }
}

pub fn styled_button(values: StyledButton) -> Node<Msg> {
    let StyledButton {
        text,
        variant,
        icon_only,
        disabled,
        active,
        icon,
    } = values;
    let mut class_list = vec!["styledButton".to_string(), variant.to_string()];
    if icon_only {
        class_list.push("iconOnly".to_string());
    }
    if active {
        class_list.push("isActive".to_string());
    }
    if icon.is_some() {
        class_list.push("withIcon".to_string());
    }

    let icon_node = match icon {
        None => empty![],
        Some(i) => styled_icon(i),
    };

    button![
        attrs![
            At::Class => class_list.join(" "),
        ],
        match disabled {
            true => vec![attrs![At::Disabled => true]],
            false => vec![],
        },
        icon_node,
        span![attrs![At::Class => "text"], text.unwrap_or_default()],
    ]
}
