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
pub struct StyledModal<'l> {
    variant: Variant,
    width: Option<usize>,
    with_icon: bool,
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
}

impl<'l> ToNode for StyledModal<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl<'l> StyledModal<'l> {
    pub fn build() -> StyledModalBuilder<'l> {
        Default::default()
    }
}

#[derive(Default)]
pub struct StyledModalBuilder<'l> {
    variant: Option<Variant>,
    width: Option<usize>,
    with_icon: Option<bool>,
    children: Option<Vec<Node<Msg>>>,
    class_list: Vec<&'l str>,
}

impl<'l> StyledModalBuilder<'l> {
    #[inline]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    #[inline]
    pub fn center(self) -> Self {
        self.variant(Variant::Center)
    }

    #[inline]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    #[inline]
    pub fn child(mut self, child: Node<Msg>) -> Self {
        self.children.get_or_insert(vec![]).push(child);
        self
    }

    #[inline]
    pub fn children<ChildIter>(mut self, children: ChildIter) -> Self
    where
        ChildIter: Iterator<Item = Node<Msg>>,
    {
        self.children.get_or_insert(vec![]).extend(children);
        self
    }

    #[inline]
    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    #[inline]
    pub fn build(self) -> StyledModal<'l> {
        StyledModal {
            variant: self.variant.unwrap_or(Variant::Center),
            width: self.width,
            with_icon: self.with_icon.unwrap_or_default(),
            children: self.children.unwrap_or_default(),
            class_list: self.class_list,
        }
    }
}

#[inline]
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
            .add_class(variant.to_icon_class_name())
            .build()
            .into_node()
    } else {
        empty![]
    };

    let close_handler = mouse_ev(Ev::Click, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        Msg::ModalDropped
    });
    let body_handler = mouse_ev(Ev::Click, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        None as Option<Msg>
    });

    let clickable_class = format!("clickableOverlay {}", variant.to_class_name());
    class_list.push("styledModal");
    class_list.push(variant.to_class_name());
    let styled_modal_style = match width {
        Some(0) => "".to_string(),
        Some(n) => format!("max-width: {width}px", width = n),
        _ => format!("max-width: {width}px", width = 130),
    };
    div![
        C!["modal"],
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
