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
    #[inline]
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            Variant::Normal => "normal",
            Variant::Primary => "primary",
        }
    }
}

impl ToString for Variant {
    #[inline]
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct StyledInputState {
    id: FieldId,
    touched: bool,
    pub value: String,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl StyledInputState {
    #[inline]
    pub fn new<S>(id: FieldId, value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id,
            touched: false,
            value: value.into(),
            min: None,
            max: None,
        }
    }

    #[inline]
    pub fn with_min(mut self, min: Option<usize>) -> Self {
        self.min = min;
        self
    }

    #[inline]
    pub fn with_max(mut self, max: Option<usize>) -> Self {
        self.max = max;
        self
    }

    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        self.value.parse::<i32>().ok()
    }

    #[inline]
    pub fn to_f64(&self) -> Option<f64> {
        self.value.parse::<f64>().ok()
    }

    #[inline]
    pub fn represent_f64_as_i32(&self) -> Option<i32> {
        self.to_f64().map(|f| (f * 10.0f64) as i32)
    }

    #[inline]
    pub fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::StrInputChanged(field_id, s) if field_id == &self.id => {
                self.value = s.clone();
                self.touched = true;
            }
            _ => (),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.value.clear();
    }
}

#[derive(Debug)]
pub struct StyledInput<'l, 'm: 'l> {
    id: FieldId,
    icon: Option<Icon>,
    valid: bool,
    value: Option<&'m str>,
    input_type: Option<&'l str>,
    input_class_list: Vec<&'l str>,
    wrapper_class_list: Vec<&'l str>,
    variant: Variant,
    auto_focus: bool,
    input_handlers: Vec<EventHandler<Msg>>,
}

impl<'l, 'm: 'l> StyledInput<'l, 'm> {
    #[inline]
    pub fn build() -> StyledInputBuilder<'l, 'm> {
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
pub struct StyledInputBuilder<'l, 'm: 'l> {
    icon: Option<Icon>,
    valid: Option<bool>,
    value: Option<&'m str>,
    input_type: Option<&'l str>,
    input_class_list: Vec<&'l str>,
    wrapper_class_list: Vec<&'l str>,
    variant: Variant,
    auto_focus: bool,
    input_handlers: Vec<EventHandler<Msg>>,
}

impl<'l, 'm: 'l> StyledInputBuilder<'l, 'm> {
    #[inline]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[inline]
    pub fn valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }

    #[inline]
    pub fn value(mut self, v: &'m str) -> Self {
        self.value = Some(v);
        self
    }

    #[inline]
    pub fn state(self, state: &'m StyledInputState) -> Self {
        self.value(&state.value.as_str()).valid(
            match (
                state.touched,
                state.value.as_str(),
                state.min.as_ref(),
                state.max.as_ref(),
            ) {
                (false, ..) => true,
                (_, s, None, None) => !s.is_empty(),
                (_, s, Some(min), None) => s.len() >= *min,
                (_, s, None, Some(max)) => s.len() <= *max,
                (_, s, Some(min), Some(max)) => s.len() >= *min && s.len() <= *max,
            },
        )
    }

    #[inline]
    pub fn add_input_class(mut self, name: &'l str) -> Self {
        self.input_class_list.push(name);
        self
    }

    #[inline]
    pub fn add_wrapper_class(mut self, name: &'l str) -> Self {
        self.wrapper_class_list.push(name);
        self
    }

    #[inline]
    pub fn primary(mut self) -> Self {
        self.variant = Variant::Primary;
        self
    }

    #[inline]
    pub fn auto_focus(mut self) -> Self {
        self.auto_focus = true;
        self
    }

    #[inline]
    pub fn on_input_ev(mut self, handler: EventHandler<Msg>) -> Self {
        self.input_handlers.push(handler);
        self
    }

    #[inline]
    pub fn build(self, id: FieldId) -> StyledInput<'l, 'm> {
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

impl<'l, 'm: 'l> ToNode for StyledInput<'l, 'm> {
    #[inline]
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
        input_class_list,
        wrapper_class_list,
        variant,
        auto_focus,
        input_handlers,
    } = values;

    let icon_node = icon
        .map(|icon| StyledIcon::build(icon).build().into_node())
        .unwrap_or(Node::Empty);

    let on_change = {
        let field_id = id.clone();
        ev(Ev::Change, move |event| {
            event.stop_propagation();
            let target = event.target().unwrap();
            Msg::StrInputChanged(field_id, seed::to_input(&target).value())
        })
    };

    div![
        C!["styledInput"],
        C![variant.to_str()],
        if !valid { Some(C!["invalid"]) } else { None },
        attrs!(
            "class" => format!("{} {}", id, wrapper_class_list.join(" ")),
        ),
        icon_node,
        seed::input![
            C!["inputElement"],
            icon.as_ref().map(|_| C!["withIcon"]),
            C![variant.to_str()],
            attrs![
                "id" => format!("{}", id),
                At::Class => input_class_list.join(" "),
                "value" => value.unwrap_or_default(),
                "type" => input_type.unwrap_or("text"),
            ],
            if auto_focus {
                vec![attrs![At::AutoFocus => true]]
            } else {
                vec![]
            },
            on_change,
            input_handlers,
        ],
    ]
}
