use seed::{prelude::*, *};

use crate::model::Icon;
use crate::shared::{styled_icon, ToNode};
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

pub struct StyledButton {
    pub variant: Variant,
    pub icon_only: bool,
    pub disabled: bool,
    pub active: bool,
    pub text: Option<String>,
    pub icon: Option<Icon>,
    pub on_click: Option<EventHandler<Msg>>,
    pub children: Vec<Node<Msg>>,
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
        icon_only,
        disabled,
        active,
        icon,
        on_click,
        children,
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
    let handler = match on_click {
        Some(h) if !disabled => vec![h],
        _ => vec![],
    };

    let icon_node = match icon {
        None => empty![],
        Some(i) => styled_icon(i),
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
        span![
            attrs![At::Class => "text"],
            text.unwrap_or_default(),
            children
        ],
    ]
}
