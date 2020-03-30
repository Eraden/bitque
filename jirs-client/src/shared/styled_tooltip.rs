use seed::{prelude::*, *};

use crate::Msg;

pub struct StyledTooltip {
    pub visible: bool,
    pub class_name: String,
    pub children: Node<Msg>,
}

impl Into<Node<Msg>> for StyledTooltip {
    fn into(self) -> Node<Msg> {
        styled_tooltip(self)
    }
}

impl StyledTooltip {
    pub fn into_node(self) -> Node<Msg> {
        self.into()
    }
}

pub fn styled_tooltip(values: StyledTooltip) -> Node<Msg> {
    let StyledTooltip {
        visible,
        class_name,
        children,
    } = values;
    if visible {
        div![
            attrs![At::Class => format!("styledTooltip {}", class_name)],
            children
        ]
    } else {
        empty!()
    }
}
