use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledInput {
    id: FieldId,
    icon: Option<Icon>,
    valid: bool,
    value: Option<String>,
    input_type: Option<String>,
}

impl StyledInput {
    pub fn build(id: FieldId) -> StyledInputBuilder {
        StyledInputBuilder {
            id,
            icon: None,
            valid: None,
            value: None,
            input_type: None,
        }
    }
}

#[derive(Debug)]
pub struct StyledInputBuilder {
    id: FieldId,
    icon: Option<Icon>,
    valid: Option<bool>,
    value: Option<String>,
    input_type: Option<String>,
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

    pub fn value<S>(mut self, v: S) -> Self
    where
        S: Into<String>,
    {
        self.value = Some(v.into());
        self
    }

    pub fn build(self) -> StyledInput {
        StyledInput {
            id: self.id,
            icon: self.icon,
            valid: self.valid.unwrap_or_default(),
            value: self.value,
            input_type: self.input_type,
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
        value,
        input_type,
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
    let field_id = id.clone();
    let change_handler = ev(Ev::Input, move |event| {
        event.stop_propagation();
        let target = event.target().unwrap();
        let input = seed::to_input(&target);
        let value = input.value();
        Msg::InputChanged(field_id, value)
    });
    let key_handler = ev(Ev::KeyUp, move |event| {
        event.stop_propagation();
        Msg::NoOp
    });

    div![
        attrs!(At::Class => wrapper_class_list.join(" ")),
        icon,
        seed::input![
            attrs![
                At::Class => input_class_list.join(" "),
                At::Value => value.unwrap_or_default(),
                At::Type => input_type.unwrap_or_else(|| "text".to_string()),
            ],
            change_handler,
            key_handler,
        ],
    ]
}
