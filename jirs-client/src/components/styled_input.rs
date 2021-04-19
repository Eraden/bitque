use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::{FieldId, Msg};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum InputVariant {
    Normal,
    Primary,
}

impl InputVariant {
    #[inline]
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            InputVariant::Normal => "normal",
            InputVariant::Primary => "primary",
        }
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
    #[inline(always)]
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

    #[inline(always)]
    pub fn with_min(mut self, min: Option<usize>) -> Self {
        self.min = min;
        self
    }

    #[inline(always)]
    pub fn with_max(mut self, max: Option<usize>) -> Self {
        self.max = max;
        self
    }

    #[inline(always)]
    pub fn to_i32(&self) -> Option<i32> {
        self.value.parse::<i32>().ok()
    }

    #[inline(always)]
    pub fn to_f64(&self) -> Option<f64> {
        self.value.parse::<f64>().ok()
    }

    #[inline(always)]
    pub fn represent_f64_as_i32(&self) -> Option<i32> {
        self.to_f64().map(|f| (f * 10.0f64) as i32)
    }

    #[inline(always)]
    pub fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::StrInputChanged(field_id, s) if field_id == &self.id => {
                self.value = s.clone();
                self.touched = true;
            }
            _ => (),
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.value.clear();
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        match (
            self.touched,
            self.value.as_str(),
            self.min.as_ref(),
            self.max.as_ref(),
        ) {
            (false, ..) => true,
            (_, s, None, None) => !s.is_empty(),
            (_, s, Some(min), None) => s.len() >= *min,
            (_, s, None, Some(max)) => s.len() <= *max,
            (_, s, Some(min), Some(max)) => s.len() >= *min && s.len() <= *max,
        }
    }
}

#[derive(Debug)]
pub struct StyledInput<'l, 'm: 'l> {
    pub id: Option<FieldId>,
    pub icon: Option<Icon>,
    pub valid: bool,
    pub value: &'m str,
    pub input_type: Option<&'l str>,
    pub input_class_list: &'l str,
    pub wrapper_class_list: &'l str,
    pub variant: InputVariant,
    pub auto_focus: bool,
    pub input_handlers: Vec<EventHandler<Msg>>,
}

impl<'l, 'm: 'l> Default for StyledInput<'l, 'm> {
    fn default() -> Self {
        Self {
            id: None,
            icon: None,
            valid: false,
            value: "",
            input_type: None,
            input_class_list: "",
            wrapper_class_list: "",
            variant: InputVariant::Normal,
            auto_focus: false,
            input_handlers: vec![],
        }
    }
}

impl<'l, 'm: 'l> StyledInput<'l, 'm> {
    #[inline(always)]
    pub fn new_with_id_and_value_and_valid(id: FieldId, value: &'m str, valid: bool) -> Self {
        Self {
            id: Some(id),
            icon: None,
            valid,
            value,
            input_type: None,
            input_class_list: "",
            wrapper_class_list: "",
            variant: InputVariant::Normal,
            auto_focus: false,
            input_handlers: vec![],
        }
    }
}

impl<'l, 'm: 'l> StyledInput<'l, 'm> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
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
        } = self;
        let id = id.expect("Input id is required");

        let icon_node = icon
            .map(|icon| StyledIcon::from(icon).render())
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
            C![
                "styledInput",
                format!("{}", id),
                variant.to_str(),
                wrapper_class_list
            ],
            IF![!valid => C!["invalid"]],
            icon_node,
            seed::input![
                C![
                    "inputElement",
                    variant.to_str(),
                    input_class_list,
                    icon.as_ref().map(|_| "withIcon").unwrap_or_default()
                ],
                attrs![
                    "id" => format!("{}", id),
                    "value" => value,
                    "type" => input_type.unwrap_or("text"),
                ],
                IF![auto_focus => attrs![At::AutoFocus => true]],
                on_change,
                input_handlers,
            ],
        ]
    }
}
