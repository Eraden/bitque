use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::Msg;

pub struct StyledInput {
    pub id: Option<String>,
    pub icon: Option<Icon>,
    pub valid: bool,
    pub on_change: EventHandler<Msg>,
}

impl ToNode for StyledInput {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledInput) -> Node<Msg> {
    let StyledInput {
        id,
        icon,
        valid,
        on_change,
    } = values;

    let mut wrapper_class_list = vec!["styledInput"];
    if !valid {
        wrapper_class_list.push("invalid");
    }

    let mut input_class_list = vec!["inputElement"];
    if icon.is_some() {
        input_class_list.push("withIcon");
    }

    let icon = match icon {
        Some(icon) => StyledIcon::build(icon).build().into_node(),
        _ => empty![],
    };

    div![
        id![id.unwrap_or_default()],
        attrs!(At::Class => wrapper_class_list.join(" ")),
        icon,
        input![attrs![At::Class => input_class_list.join(" ")], on_change]
    ]
}
