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
    pub field_id: FieldId,
    pub name: &'l str,
    pub label: &'l str,
    pub value: u32,
    pub selected: bool,
    pub class_list: &'l str,
}

impl<'l> Default for ChildBuilder<'l> {
    fn default() -> Self {
        Self {
            field_id: FieldId::TextFilterBoard,
            name: "",
            label: "",
            value: 0,
            selected: false,
            class_list: "",
        }
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

        let id = field_id.to_string();
        let field_id_clone = field_id.clone();
        let handler: EventHandler<Msg> = mouse_ev(Ev::Click, move |_| {
            Msg::U32InputChanged(field_id_clone, value)
        });

        div![
            C![
                "styledCheckboxChild",
                class_list,
                IF![selected => "selected"]
            ],
            handler,
            label![attrs![At::For => format!("{}-{}", id, name)], label],
            input![
                attrs![At::Type => "radio", At::Name => name, At::Id => format!("{}-{}", id, name)],
                IF![selected => attrs!(At::Checked => selected)]
            ],
        ]
    }
}

#[derive(Debug)]
pub struct StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    pub options: Option<Options>,
    pub class_list: &'l str,
}

impl<'l, Options> Default for StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    fn default() -> Self {
        Self {
            options: None,
            class_list: "",
        }
    }
}

impl<'l, Options> ToNode for StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

fn render<'l, Options>(values: StyledCheckbox<'l, Options>) -> Node<Msg>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    let StyledCheckbox {
        options,
        class_list,
    } = values;

    let opt: Vec<Node<Msg>> = match options {
        Some(options) => options.map(|child| child.into_node()).collect(),
        _ => vec![Node::Empty],
    };

    div![C!["styledCheckbox", class_list], opt,]
}
