use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Variant {
    Normal,
    Primary,
}

impl Variant {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            Variant::Normal => "normal",
            Variant::Primary => "primary",
        }
    }
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct StyledInputState {
    id: FieldId,
    touched: bool,
    pub value: String,
}

impl StyledInputState {
    pub fn new<S>(id: FieldId, value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id,
            touched: false,
            value: value.into(),
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        self.value.parse::<i32>().ok()
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.value.parse::<f64>().ok()
    }

    pub fn represent_f64_as_i32(&self) -> Option<i32> {
        self.to_f64().map(|f| (f * 10.0f64) as i32)
    }

    pub fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::StrInputChanged(field_id, s) if field_id == &self.id => {
                self.value = s.clone();
                self.touched = true;
            }
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.value.clear();
    }
}

#[derive(Debug)]
pub struct StyledInput<'l> {
    id: FieldId,
    icon: Option<Icon>,
    valid: bool,
    value: Option<String>,
    input_type: Option<&'l str>,
    input_class_list: Vec<&'l str>,
    wrapper_class_list: Vec<&'l str>,
    variant: Variant,
    auto_focus: bool,
    input_handlers: Vec<EventHandler<Msg>>,
}

impl<'l> StyledInput<'l> {
    pub fn build() -> StyledInputBuilder<'l> {
        StyledInputBuilder {
            icon: None,
            valid: None,
            value: None,
            input_type: None,
            input_class_list: vec![],
            wrapper_class_list: vec![],
            variant: Variant::Normal,
            auto_focus: false,
            input_handlers: vec![],
        }
    }
}

#[derive(Debug)]
pub struct StyledInputBuilder<'l> {
    icon: Option<Icon>,
    valid: Option<bool>,
    value: Option<String>,
    input_type: Option<&'l str>,
    input_class_list: Vec<&'l str>,
    wrapper_class_list: Vec<&'l str>,
    variant: Variant,
    auto_focus: bool,
    input_handlers: Vec<EventHandler<Msg>>,
}

impl<'l> StyledInputBuilder<'l> {
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

    pub fn state(self, state: &StyledInputState) -> Self {
        self.value(state.value.as_str())
            .valid(!state.touched || !state.value.is_empty())
    }

    pub fn add_input_class(mut self, name: &'l str) -> Self {
        self.input_class_list.push(name);
        self
    }

    pub fn add_wrapper_class(mut self, name: &'l str) -> Self {
        self.wrapper_class_list.push(name);
        self
    }

    pub fn primary(mut self) -> Self {
        self.variant = Variant::Primary;
        self
    }

    pub fn auto_focus(mut self) -> Self {
        self.auto_focus = true;
        self
    }

    pub fn on_input_ev(mut self, handler: EventHandler<Msg>) -> Self {
        self.input_handlers.push(handler);
        self
    }

    pub fn build(self, id: FieldId) -> StyledInput<'l> {
        StyledInput {
            id,
            icon: self.icon,
            valid: self.valid.unwrap_or_default(),
            value: self.value,
            input_type: self.input_type,
            input_class_list: self.input_class_list,
            wrapper_class_list: self.wrapper_class_list,
            variant: self.variant,
            auto_focus: self.auto_focus,
            input_handlers: self.input_handlers,
        }
    }
}

impl<'l> ToNode for StyledInput<'l> {
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
        mut input_class_list,
        mut wrapper_class_list,
        variant,
        auto_focus,
        input_handlers,
    } = values;

    wrapper_class_list.push(variant.to_str());
    if !valid {
        wrapper_class_list.push("invalid");
    }

    input_class_list.push(variant.to_str());
    if icon.is_some() {
        input_class_list.push("withIcon");
    }

    let icon = match icon {
        Some(icon) => StyledIcon::build(icon).build().into_node(),
        _ => empty![],
    };
    let on_input = {
        let field_id = id.clone();
        ev(Ev::Input, move |event| {
            event.stop_propagation();
            let target = event.target().unwrap();
            let input = seed::to_input(&target);
            let value = input.value();
            Msg::StrInputChanged(field_id, value)
        })
    };
    let on_keyup = {
        ev(Ev::KeyUp, move |event| {
            event.stop_propagation();
            None as Option<Msg>
        })
    };
    let on_click = {
        ev(Ev::Click, move |event| {
            event.stop_propagation();
            None as Option<Msg>
        })
    };

    div![
        C!["styledInput"],
        attrs!(
            At::Class => wrapper_class_list.join(" "),
            At::Class => format!("{}", id),
        ),
        icon,
        on_click,
        on_keyup,
        seed::input![
            C!["inputElement"],
            attrs![
                At::Id => format!("{}", id),
                At::Class => input_class_list.join(" "),
                At::Value => value.unwrap_or_default(),
                At::Type => input_type.unwrap_or_else(|| "text"),

            ],
            if auto_focus {
                vec![attrs![At::AutoFocus => true]]
            } else {
                vec![]
            },
            on_input,
            input_handlers,
        ],
    ]
}
