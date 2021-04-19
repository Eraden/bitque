use seed::prelude::*;
use seed::*;

use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledCheckboxState {
    pub field_id: FieldId,
    pub value: u32,
}

impl StyledCheckboxState {
    #[inline(always)]
    pub fn new(field_id: FieldId, value: u32) -> Self {
        Self { field_id, value }
    }

    #[inline(always)]
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
    #[inline(always)]
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

impl<'l> ChildBuilder<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let ChildBuilder {
            field_id,
            name,
            label,
            value,
            selected,
            class_list,
        } = self;

        let id = field_id.to_string();
        let handler: EventHandler<Msg> = {
            let id = field_id;
            mouse_ev(Ev::Click, move |_| Msg::U32InputChanged(id, value))
        };

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
    #[inline(always)]
    fn default() -> Self {
        Self {
            options: None,
            class_list: "",
        }
    }
}

impl<'l, Options> StyledCheckbox<'l, Options>
where
    Options: Iterator<Item = ChildBuilder<'l>>,
{
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledCheckbox {
            options,
            class_list,
        } = self;

        div![
            C!["styledCheckbox", class_list],
            options.map_or_else(
                || vec![Node::Empty],
                |v| v.map(ChildBuilder::render).collect(),
            )
        ]
    }
}
