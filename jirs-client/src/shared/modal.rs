use seed::{prelude::*, *};

use crate::model::Icon;
use crate::shared::{styled_icon, ToNode};
use crate::Msg;

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
    div![
        attrs![At::Class => "modal"],
        div![
            attrs![At::Class => format!("clickableOverlay {}", variant.to_class_name())],
            div![
                attrs![At::Class => format!("styledModal {}", variant.to_class_name())],
                icon,
                children
            ]
        ]
    ]
}
