use seed::prelude::*;
use seed::*;

use super::events;
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
    pub icon: Node<Msg>,
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
            icon: Node::Empty,
        }
    }
}

impl<'l> ChildBuilder<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let id = self.field_id.to_string();
        let handler = events::on_click_change_number_input(self.field_id, self.value);

        div![
            C![
                "styledCheckboxChild",
                self.class_list,
                IF![self.selected => "selected"]
            ],
            handler,
            self.icon,
            label![
                attrs![At::For => format!("{}-{}", id, self.name)],
                self.label
            ],
            input![
                attrs![At::Type => "radio", At::Name => self.name, At::Id => format!("{}-{}", id, self.name)],
                IF![self.selected => attrs!(At::Checked => self.selected)]
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
