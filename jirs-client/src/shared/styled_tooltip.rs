use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

pub struct StyledTooltip {
    pub visible: bool,
    pub class_name: String,
    pub children: Node<Msg>,
}

impl ToNode for StyledTooltip {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledTooltip) -> Node<Msg> {
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
