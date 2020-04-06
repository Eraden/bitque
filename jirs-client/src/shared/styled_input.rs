use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledInput {
    id: FieldId,
    icon: Option<Icon>,
    valid: bool,
    on_change: Option<EventHandler<Msg>>,
}

impl StyledInput {
    pub fn build(id: FieldId) -> StyledInputBuilder {
        StyledInputBuilder {
            id,
            icon: None,
            valid: None,
            on_change: None,
        }
    }
}

#[derive(Debug)]
pub struct StyledInputBuilder {
    id: FieldId,
    icon: Option<Icon>,
    valid: Option<bool>,
    on_change: Option<EventHandler<Msg>>,
}

impl StyledInputBuilder {
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }

    pub fn on_change(mut self, on_change: EventHandler<Msg>) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn build(self) -> StyledInput {
        StyledInput {
            id: self.id,
            icon: self.icon,
            valid: self.valid.unwrap_or_default(),
            on_change: self.on_change,
        }
    }
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

    let mut wrapper_class_list = vec!["styledInput".to_string(), format!("{}", id)];
    if !valid {
        wrapper_class_list.push("invalid".to_string());
    }

    let mut input_class_list = vec!["inputElement".to_string()];
    if icon.is_some() {
        input_class_list.push("withIcon".to_string());
    }

    let icon = match icon {
        Some(icon) => StyledIcon::build(icon).build().into_node(),
        _ => empty![],
    };

    let mut handlers = vec![];

    if let Some(handler) = on_change {
        handlers.push(handler);
    }
    let input_handler = input_ev(Ev::KeyPress, move |value| Msg::InputChanged(id, value));
    handlers.push(input_handler);

    div![
        attrs!(At::Class => wrapper_class_list.join(" ")),
        icon,
        seed::input![attrs![At::Class => input_class_list.join(" ")], handlers],
    ]
}
