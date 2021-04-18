use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::Msg;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ModalVariant {
    Center,
    Aside,
}

impl ModalVariant {
    pub fn to_class_name(&self) -> &str {
        match self {
            ModalVariant::Center => "center",
            ModalVariant::Aside => "aside",
        }
    }

    pub fn to_icon_class_name(&self) -> &str {
        match self {
            ModalVariant::Center => "modalVariantCenter",
            ModalVariant::Aside => "modalVariantAside",
        }
    }
}

#[derive(Debug)]
pub struct StyledModal<'l> {
    pub variant: ModalVariant,
    pub width: Option<usize>,
    pub with_icon: bool,
    pub children: Vec<Node<Msg>>,
    pub class_list: &'l str,
}

impl<'l> Default for StyledModal<'l> {
    fn default() -> Self {
        Self {
            variant: ModalVariant::Center,
            width: None,
            with_icon: false,
            children: vec![],
            class_list: "",
        }
    }
}

impl<'l> StyledModal<'l> {
    pub fn centered_with_width_and_body(width: usize, children: Vec<Node<Msg>>) -> Self {
        Self {
            variant: ModalVariant::Center,
            width: Some(width),
            with_icon: false,
            children,
            class_list: "",
        }
    }
}

impl<'l> ToNode for StyledModal<'l> {
    fn into_node(self) -> Node<Msg> {
        self.render()
    }
}

impl<'l> StyledModal<'l> {
    #[inline]
    pub fn render(self) -> Node<Msg> {
        let StyledModal {
            variant,
            width,
            with_icon,
            children,
            class_list,
        } = self;

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

        let styled_modal_style = match width {
            Some(0) => "".to_string(),
            Some(n) => format!("max-width: {width}px", width = n),
            _ => format!("max-width: {width}px", width = 130),
        };
        div![
            C!["modal"],
            div![
                C!["clickableOverlay", variant.to_class_name()],
                close_handler,
                div![
                    C![class_list, "styledModal", variant.to_class_name()],
                    attrs![At::Style => styled_modal_style],
                    body_handler,
                    IF![with_icon => StyledIcon {
                        icon: Icon::Close,
                        class_list: variant.to_icon_class_name(),
                        ..Default::default()
                    }
                    .render()],
                    children
                ]
            ]
        ]
    }
}
