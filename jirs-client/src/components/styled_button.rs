use {
    crate::{shared::ToNode, ButtonId, Msg},
    seed::{prelude::*, *},
};

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
    button_id: Option<ButtonId>,
}

impl<'l> StyledButtonBuilder<'l> {
    #[inline(always)]
    fn variant(mut self, value: Variant) -> Self {
        self.variant = Some(value);
        self
    }

    #[inline(always)]
    pub fn primary(self) -> Self {
        self.variant(Variant::Primary)
    }

    #[inline(always)]
    pub fn success(self) -> Self {
        self.variant(Variant::Success)
    }

    #[inline(always)]
    pub fn danger(self) -> Self {
        self.variant(Variant::Danger)
    }

    #[inline(always)]
    pub fn secondary(self) -> Self {
        self.variant(Variant::Secondary)
    }

    #[inline(always)]
    pub fn empty(self) -> Self {
        self.variant(Variant::Empty)
    }

    // pub fn button_id(mut self, button_id: ButtonId) -> Self {
    //     self.button_id = Some(button_id);
    //     self
    // }

    #[inline(always)]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    #[inline(always)]
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    #[inline(always)]
    pub fn text(mut self, value: &'l str) -> Self {
        self.text = Some(value);
        self
    }

    #[inline(always)]
    pub fn icon<I>(mut self, value: I) -> Self
    where
        I: ToNode,
    {
        self.icon = Some(value.into_node());
        self
    }

    #[inline(always)]
    pub fn on_click(mut self, value: EventHandler<Msg>) -> Self {
        self.on_click = Some(value);
        self
    }

    #[inline(always)]
    pub fn children(mut self, value: Vec<Node<Msg>>) -> Self {
        self.children = Some(value);
        self
    }

    #[inline(always)]
    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    #[inline(always)]
    pub fn set_type_reset(mut self) -> Self {
        self.button_type = Some("reset");
        self
    }

    #[inline(always)]
    pub fn build(self) -> StyledButton<'l> {
        StyledButton {
            variant: self.variant.unwrap_or(Variant::Primary),
            disabled: self.disabled.unwrap_or(false),
            active: self.active.unwrap_or(false),
            text: self.text,
            icon: self.icon,
            on_click: self.on_click,
            children: self.children.unwrap_or_default(),
            class_list: self.class_list,
            button_type: self.button_type.unwrap_or("submit"),
            button_id: self.button_id,
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
    button_id: Option<ButtonId>,
}

impl<'l> StyledButton<'l> {
    pub fn secondary_with_text_and_icon<I>(text: &'l str, icon: I) -> Self
    where
        I: ToNode,
    {
        Self {
            variant: Variant::Secondary,
            disabled: false,
            active: false,
            text: Some(text),
            icon: Some(icon.into_node()),
            on_click: None,
            children: vec![],
            class_list: vec![],
            button_type: "",
            button_id: None,
        }
    }
}

impl<'l> StyledButton<'l> {
    #[inline(always)]
    pub fn build() -> StyledButtonBuilder<'l> {
        StyledButtonBuilder::default()
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
        mut class_list,
        button_type,
        button_id,
    } = values;
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

    let icon_node = icon.unwrap_or(Node::Empty);
    let content = if children.is_empty() && text.is_none() {
        Node::Empty
    } else {
        span![C!["text"], text.unwrap_or_default(), children]
    };

    let class_list: Attrs = {
        let class_list: String = class_list.join(" ");
        C![class_list.as_str()]
    };
    let button_id = button_id.map(|id| id.to_str()).unwrap_or_default();

    seed::button![
        C!["styledButton"],
        class_list,
        attrs![
            At::Id => button_id,
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
