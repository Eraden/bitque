use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug)]
pub struct StyledInput {
    id: Option<String>,
    icon: Option<Icon>,
    valid: bool,
    on_change: Option<EventHandler<Msg>>,
}

impl StyledInput {
    pub fn build() -> StyledInputBuilder {
        StyledInputBuilder::default()
    }
}

#[derive(Default, Debug)]
pub struct StyledInputBuilder {
    id: Option<String>,
    icon: Option<Icon>,
    valid: Option<bool>,
    on_change: Option<EventHandler<Msg>>,
}

impl StyledInputBuilder {
    pub fn id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.id = Some(id.into());
        self
    }

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

    let input_node = match on_change {
        Some(on_change) => seed::input![attrs![At::Class => input_class_list.join(" ")], on_change],
        _ => seed::input![attrs![At::Class => input_class_list.join(" ")]],
    };

    div![
        id![id.unwrap_or_default()],
        attrs!(At::Class => wrapper_class_list.join(" ")),
        icon,
        input_node,
    ]
}
