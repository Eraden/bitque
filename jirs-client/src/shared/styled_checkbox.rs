use seed::{prelude::*, *};

use jirs_data::TimeTracking;

use crate::shared::{ToChild, ToNode};
use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledCheckboxState {
    pub field_id: FieldId,
    pub value: u32,
}

impl StyledCheckboxState {
    pub fn new(field_id: FieldId, value: u32) -> Self {
        Self { field_id, value }
    }

    pub fn update(&mut self, msg: &Msg) {
        if let Msg::U32InputChanged(field_id, value) = msg {
            if field_id != &self.field_id {
                return;
            }
            self.value = *value;
        }
    }
}

#[derive(Debug)]
pub struct ChildBuilder {
    field_id: Option<FieldId>,
    name: String,
    label: String,
    value: u32,
    selected: bool,
    class_list: Vec<String>,
}

impl Default for ChildBuilder {
    fn default() -> Self {
        Self {
            field_id: None,
            name: "".to_string(),
            label: "".to_string(),
            value: 0,
            selected: false,
            class_list: vec![],
        }
    }
}

impl ChildBuilder {
    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.label = label.into();
        self
    }

    pub fn with_id(mut self, id: FieldId) -> Self {
        self.field_id = Some(id);
        self
    }

    pub fn try_select(mut self, value: u32) -> Self {
        self.selected = self.value == value;
        self
    }

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }
}

impl ToNode for ChildBuilder {
    fn into_node(self) -> Node<Msg> {
        let ChildBuilder {
            field_id,
            name,
            label,
            value,
            selected,
            mut class_list,
        } = self;

        let id = field_id.as_ref().map(|f| f.to_string()).unwrap_or_default();
        let field_id_clone = field_id.as_ref().cloned();
        let handler: EventHandler<Msg> = mouse_ev(Ev::Click, move |_| {
            field_id_clone.map(|field_id| Msg::U32InputChanged(field_id, value))
        });

        class_list.push("styledCheckboxChild".to_string());
        class_list.push(if selected { "selected" } else { "" }.to_string());

        let input_attrs = if selected {
            attrs![At::Type => "radio", At::Name => name.as_str(), At::Checked => selected, At::Id => format!("{}-{}", id, name)]
        } else {
            attrs![At::Type => "radio", At::Name => name.as_str(), At::Id => format!("{}-{}", id, name)]
        };

        div![
            attrs![At::Class => class_list.join(" ")],
            handler,
            label![attrs![At::For => format!("{}-{}", id, name)], label],
            input![input_attrs],
        ]
    }
}

#[derive(Debug)]
pub struct StyledCheckbox {
    id: FieldId,
    options: Vec<ChildBuilder>,
    selected: u32,
    class_list: Vec<String>,
}

impl ToNode for StyledCheckbox {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledCheckbox {
    pub fn build() -> StyledCheckboxBuilder {
        StyledCheckboxBuilder {
            options: vec![],
            selected: 0,
            class_list: vec![],
        }
    }
}

pub struct StyledCheckboxBuilder {
    options: Vec<ChildBuilder>,
    selected: u32,
    class_list: Vec<String>,
}

impl StyledCheckboxBuilder {
    pub fn state(mut self, state: &StyledCheckboxState) -> Self {
        self.selected = state.value;
        self
    }

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn options(mut self, options: Vec<ChildBuilder>) -> Self {
        self.options = options;
        self
    }

    pub fn build(self, field_id: FieldId) -> StyledCheckbox {
        StyledCheckbox {
            id: field_id,
            options: self.options,
            selected: self.selected,
            class_list: self.class_list,
        }
    }
}

fn render(values: StyledCheckbox) -> Node<Msg> {
    let StyledCheckbox {
        id,
        options,
        selected,
        class_list,
    } = values;

    let opt: Vec<Node<Msg>> = options
        .into_iter()
        .map(|child| child.with_id(id.clone()).try_select(selected).into_node())
        .collect();

    div![
        class!["styledCheckbox"],
        attrs![At::Class => class_list.join(" ")],
        opt,
    ]
}

impl ToChild for TimeTracking {
    type Builder = ChildBuilder;

    fn to_child(&self) -> Self::Builder {
        Self::Builder::default()
            .label(match self {
                TimeTracking::Untracked => "No tracking",
                TimeTracking::Fibonacci => "Fibonacci (Bad mode)",
                TimeTracking::Hourly => "Evil Mode (Hourly)",
            })
            .name(match self {
                TimeTracking::Untracked => "untracked",
                TimeTracking::Fibonacci => "fibonacci",
                TimeTracking::Hourly => "hourly",
            })
            .value((*self).into())
            .add_class(match self {
                TimeTracking::Untracked => "untracked",
                TimeTracking::Fibonacci => "fibonacci",
                TimeTracking::Hourly => "hourly",
            })
    }
}
