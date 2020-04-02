use seed::{prelude::*, *};

use crate::model::Icon;
use crate::shared::{styled_icon, ToNode};
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
pub struct Modal {
    pub variant: Variant,
    pub width: usize,
    pub with_icon: bool,
    pub children: Vec<Node<Msg>>,
}

impl ToNode for Modal {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: Modal) -> Node<Msg> {
    let Modal {
        variant,
        width,
        with_icon,
        children,
    } = values;

    let icon = if with_icon {
        let mut styled_icon = styled_icon(Icon::Close);
        styled_icon.add_class(variant.to_icon_class_name().to_string());
        styled_icon
    } else {
        empty![]
    };

    let close_handler = mouse_ev(Ev::Click, |_| Msg::PopModal);
    let body_handler = mouse_ev(Ev::Click, |ev| {
        ev.stop_propagation();
        Msg::NoOp
    });

    let clickable_class = format!("clickableOverlay {}", variant.to_class_name());
    let styled_modal_class = format!("styledModal {}", variant.to_class_name());
    let styled_modal_style = format!("max-width: {width}px", width = width);
    div![
        attrs![At::Class => "modal"],
        div![
            attrs![At::Class => clickable_class],
            close_handler,
            div![
                attrs![At::Class => styled_modal_class, At::Style => styled_modal_style],
                body_handler,
                icon,
                children
            ]
        ]
    ]
}
