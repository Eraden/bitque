use {
    crate::{shared::ToNode, ButtonId, Msg},
    seed::{prelude::*, *},
};

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
    pub fn secondary_with_text_and_icon<I>(text: &'l str, icon: I) -> Self
    where
        I: ToNode,
    {
        Self {
            variant: ButtonVariant::Secondary,
            disabled: false,
            active: false,
            text: Some(text),
            icon: Some(icon.into_node()),
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

impl<'l> ToNode for StyledButton<'l> {
    #[inline(always)]
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[inline(always)]
pub fn render(values: StyledButton) -> Node<Msg> {
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
    } = values;
    let class_list = format!(
        "{} {} {} {} {}",
        class_list,
        variant,
        if children.is_empty() && text.is_none() {
            "iconOnly"
        } else {
            ""
        },
        if active { "isActive" } else { "" },
        if icon.is_some() { "withIcon" } else { "" }
    );
    let handler = match on_click {
        Some(h) if !disabled => vec![h],
        _ => vec![],
    };

    let icon_node = icon.unwrap_or(Node::Empty);
    let content = if children.is_empty() && text.is_none() {
        Node::Empty
    } else {
        span![C!["text"], text.unwrap_or_default(), children]
    };

    let button_id = button_id.map(|id| id.to_str()).unwrap_or_default();

    seed::button![
        C!["styledButton", class_list],
        attrs![At::Id => button_id, At::Type => button_type],
        IF![disabled => attrs![At::Disabled => true]],
        handler,
        icon_node,
        content,
    ]
}
