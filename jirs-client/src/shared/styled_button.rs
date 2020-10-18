use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
enum Variant {
    Primary,
    Success,
    Danger,
    Secondary,
    Empty,
}

impl Variant {
    fn to_str(&self) -> &'static str {
        match self {
            Variant::Primary => "primary",
            Variant::Success => "success",
            Variant::Danger => "danger",
            Variant::Secondary => "secondary",
            Variant::Empty => "empty",
        }
    }
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Default)]
pub struct StyledButtonBuilder<'l> {
    variant: Option<Variant>,
    disabled: Option<bool>,
    active: Option<bool>,
    text: Option<&'l str>,
    icon: Option<Node<Msg>>,
    on_click: Option<EventHandler<Msg>>,
    children: Option<Vec<Node<Msg>>>,
    class_list: Vec<&'l str>,
    button_type: Option<&'l str>,
}

impl<'l> StyledButtonBuilder<'l> {
    fn variant(mut self, value: Variant) -> Self {
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

    pub fn text(mut self, value: &'l str) -> Self {
        self.text = Some(value);
        self
    }

    pub fn icon<I>(mut self, value: I) -> Self
    where
        I: ToNode,
    {
        self.icon = Some(value.into_node());
        self
    }

    pub fn on_click(mut self, value: EventHandler<Msg>) -> Self {
        self.on_click = Some(value);
        self
    }

    pub fn children(mut self, value: Vec<Node<Msg>>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn set_type_reset(mut self) -> Self {
        self.button_type = Some("reset");
        self
    }

    pub fn build(self) -> StyledButton<'l> {
        StyledButton {
            variant: self.variant.unwrap_or_else(|| Variant::Primary),
            disabled: self.disabled.unwrap_or_else(|| false),
            active: self.active.unwrap_or_else(|| false),
            text: self.text,
            icon: self.icon,
            on_click: self.on_click,
            children: self.children.unwrap_or_default(),
            class_list: self.class_list,
            button_type: self.button_type.unwrap_or_else(|| "submit"),
        }
    }
}

pub struct StyledButton<'l> {
    variant: Variant,
    disabled: bool,
    active: bool,
    text: Option<&'l str>,
    icon: Option<Node<Msg>>,
    on_click: Option<EventHandler<Msg>>,
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
    button_type: &'l str,
}

impl<'l> StyledButton<'l> {
    pub fn build() -> StyledButtonBuilder<'l> {
        StyledButtonBuilder::default()
    }
}

impl<'l> ToNode for StyledButton<'l> {
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
        mut class_list,
        button_type,
    } = values;
    class_list.push("styledButton");
    class_list.push(variant.to_str());
    if children.is_empty() && text.is_none() {
        class_list.push("iconOnly");
    }
    if active {
        class_list.push("isActive");
    }
    if icon.is_some() {
        class_list.push("withIcon");
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
            At::Type => button_type,
        ],
        handler,
        if disabled {
            vec![attrs![At::Disabled => true]]
        } else {
            vec![]
        },
        icon_node,
        content,
    ]
}
