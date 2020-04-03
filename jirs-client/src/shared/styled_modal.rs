use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Variant {
    Center,
    Aside,
}

impl Variant {
    pub fn to_class_name(&self) -> &str {
        match self {
            Variant::Center => "center",
            Variant::Aside => "aside",
        }
    }

    pub fn to_icon_class_name(&self) -> &str {
        match self {
            Variant::Center => "modalVariantCenter",
            Variant::Aside => "modalVariantAside",
        }
    }
}

#[derive(Debug)]
pub struct StyledModal {
    pub variant: Variant,
    pub width: usize,
    pub with_icon: bool,
    pub children: Vec<Node<Msg>>,
    pub class_list: Vec<String>,
}

impl ToNode for StyledModal {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledModal {
    pub fn build() -> StyledModalBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct StyledModalBuilder {
    variant: Option<Variant>,
    width: Option<usize>,
    with_icon: Option<bool>,
    children: Option<Vec<Node<Msg>>>,
    class_list: Vec<String>,
}

impl StyledModalBuilder {
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_icon(mut self, with_icon: bool) -> Self {
        self.with_icon = Some(with_icon);
        self
    }

    pub fn children(mut self, children: Vec<Node<Msg>>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn add_class(mut self, name: String) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn build(self) -> StyledModal {
        StyledModal {
            variant: self.variant.unwrap_or_else(|| Variant::Center),
            width: self.width.unwrap_or_else(|| 130),
            with_icon: self.with_icon.unwrap_or_default(),
            children: self.children.unwrap_or_default(),
            class_list: self.class_list,
        }
    }
}

pub fn render(values: StyledModal) -> Node<Msg> {
    let StyledModal {
        variant,
        width,
        with_icon,
        children,
        mut class_list,
    } = values;

    let icon = if with_icon {
        StyledIcon::build(Icon::Close)
            .add_class(variant.to_icon_class_name().to_string())
            .build()
            .into_node()
    } else {
        empty![]
    };

    let close_handler = mouse_ev(Ev::Click, |_| Msg::ModalDropped);
    let body_handler = mouse_ev(Ev::Click, |ev| {
        ev.stop_propagation();
        Msg::NoOp
    });

    let clickable_class = format!("clickableOverlay {}", variant.to_class_name());
    class_list.push(format!("styledModal {}", variant.to_class_name()));
    let styled_modal_style = format!("max-width: {width}px", width = width);
    div![
        attrs![At::Class => "modal"],
        div![
            attrs![At::Class => clickable_class],
            close_handler,
            div![
                attrs![At::Class => class_list.join(" "), At::Style => styled_modal_style],
                body_handler,
                icon,
                children
            ]
        ]
    ]
}
