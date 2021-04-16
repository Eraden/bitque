use seed::prelude::*;
use seed::*;

use crate::shared::ToNode;
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
pub struct ChildBuilder<'l> {
    pub field_id: Option<FieldId>,
    pub name: &'l str,
    pub label: &'l str,
    pub value: u32,
    pub selected: bool,
    pub class_list: &'l str,
}

impl<'l> Default for ChildBuilder<'l> {
    fn default() -> Self {
        Self {
            field_id: None,
            name: "",
            label: "",
            value: 0,
            selected: false,
            class_list: "",
        }
    }
}

impl<'l> ChildBuilder<'l> {
    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn name(mut self, name: &'l str) -> Self {
        self.name = name;
        self
    }

    pub fn label(mut self, label: &'l str) -> Self {
        self.label = label;
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

    pub fn class_list(mut self, name: &'l str) -> Self {
        self.class_list = name;
        self
    }
}

impl<'l> ToNode for ChildBuilder<'l> {
    fn into_node(self) -> Node<Msg> {
        let ChildBuilder {
            field_id,
            name,
            label,
            value,
            selected,
            class_list,
        } = self;

        let id = field_id.as_ref().map(|f| f.to_string()).unwrap_or_default();
        let field_id_clone = field_id.as_ref().cloned();
        let handler: EventHandler<Msg> = mouse_ev(Ev::Click, move |_| {
            field_id_clone.map(|field_id| Msg::U32InputChanged(field_id, value))
        });

        let input_attrs = if selected {
            attrs![At::Type => "radio", At::Name => name, At::Checked => selected, At::Id => format!("{}-{}", id, name)]
        } else {
            attrs![At::Type => "radio", At::Name => name, At::Id => format!("{}-{}", id, name)]
        };

        div![
            C![
                "styledCheckboxChild",
                class_list,
                IF![selected => "selected"]
            ],
            handler,
            label![attrs![At::For => format!("{}-{}", id, name)], label],
            input![input_attrs],
        ]
    }
}

#[derive(Debug)]
pub struct StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    id: FieldId,
    options: Option<Options>,
    selected: u32,
    class_list: Vec<&'l str>,
}

impl<'l, Options> ToNode for StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl<'l, Options> StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    pub fn build() -> StyledCheckboxBuilder<'l, Options> {
        StyledCheckboxBuilder {
            options: None,
            selected: 0,
            class_list: vec![],
        }
    }
}

pub struct StyledCheckboxBuilder<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    options: Option<Options>,
    selected: u32,
    class_list: Vec<&'l str>,
}

impl<'l, Options> StyledCheckboxBuilder<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    pub fn state(mut self, state: &StyledCheckboxState) -> Self {
        self.selected = state.value;
        self
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn options(mut self, options: Options) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build(self, field_id: FieldId) -> StyledCheckbox<'l, Options> {
        StyledCheckbox {
            id: field_id,
            options: self.options,
            selected: self.selected,
            class_list: self.class_list,
        }
    }
}

fn render<'l, Options>(values: StyledCheckbox<'l, Options>) -> Node<Msg>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    let StyledCheckbox {
        id,
        options,
        selected,
        class_list,
    } = values;

    let opt: Vec<Node<Msg>> = match options {
        Some(options) => options
            .map(|child| child.with_id(id.clone()).try_select(selected).into_node())
            .collect(),
        _ => vec![Node::Empty],
    };

    div![
        C!["styledCheckbox"],
        attrs![At::Class => class_list.join(" ")],
        opt,
    ]
}
