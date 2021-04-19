use seed::prelude::*;
use seed::*;

use crate::{ButtonId, Msg};

#[allow(dead_code)]
pub enum ButtonVariant {
    Primary,
    Success,
    Danger,
    Secondary,
    Empty,
}

impl ButtonVariant {
    fn to_str(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Success => "success",
            ButtonVariant::Danger => "danger",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Empty => "empty",
        }
    }
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

pub struct StyledButton<'l> {
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub active: bool,
    pub text: Option<&'l str>,
    pub icon: Option<Node<Msg>>,
    pub on_click: Option<EventHandler<Msg>>,
    pub children: Vec<Node<Msg>>,
    pub class_list: &'l str,
    pub button_type: &'l str,
    pub button_id: Option<ButtonId>,
}

impl<'l> StyledButton<'l> {
    pub fn secondary_with_text_and_icon(text: &'l str, icon: Node<Msg>) -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            disabled: false,
            active: false,
            text: Some(text),
            icon: Some(icon),
            on_click: None,
            children: vec![],
            class_list: "",
            button_type: "submit",
            button_id: None,
        }
    }
}

impl<'l> Default for StyledButton<'l> {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            disabled: false,
            active: false,
            text: None,
            icon: None,
            on_click: None,
            children: vec![],
            class_list: "",
            button_type: "",
            button_id: None,
        }
    }
}

impl<'l> StyledButton<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledButton {
            text,
            variant,
            disabled,
            active,
            icon,
            on_click,
            children,
            class_list,
            button_type,
            button_id,
        } = self;

        let handler = match on_click {
            Some(h) if !disabled => vec![h],
            _ => vec![],
        };

        let children_len = children.len();
        let content = if children.is_empty() && text.is_none() {
            Node::Empty
        } else {
            span![C!["text"], text.unwrap_or_default(), children]
        };

        let button_id = button_id.map(|id| id.to_str()).unwrap_or_default();

        seed::button![
            C![
                "styledButton",
                class_list,
                variant.to_str(),
                IF![children_len > 0 && text.is_none() => "iconOnly"],
                IF![active => "isActive"],
                IF![icon.is_some() => "withIcon"],
            ],
            attrs![At::Id => button_id, At::Type => button_type],
            IF![disabled => attrs![At::Disabled => true]],
            handler,
            icon.unwrap_or(Node::Empty),
            content,
        ]
    }
}
