use std::collections::HashMap;

use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select_child::*;
use crate::{FieldId, Msg};

#[derive(Clone, Debug, PartialEq)]
pub enum StyledSelectChanged {
    Text(String),
    DropDownVisibility(bool),
    Changed(Option<u32>),
    RemoveMulti(u32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum SelectVariant {
    Empty,
    Normal,
}

impl Default for SelectVariant {
    fn default() -> Self {
        SelectVariant::Empty
    }
}

impl SelectVariant {
    pub fn to_str<'l>(self) -> &'l str {
        match self {
            SelectVariant::Empty => "empty",
            SelectVariant::Normal => "normal",
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct StyledSelectState {
    pub field_id: FieldId,
    pub opened: bool,
    pub text_filter: String,
    pub values: Vec<u32>,
}

impl StyledSelectState {
    pub fn field_id(&self) -> FieldId {
        self.field_id.clone()
    }

    pub fn reset(&mut self) {
        self.text_filter.clear();
        self.opened = false;
        self.values = vec![];
    }
}

impl StyledSelectState {
    pub fn new(field_id: FieldId, values: Vec<u32>) -> Self {
        Self {
            field_id,
            opened: false,
            text_filter: String::new(),
            values,
        }
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        let field_id = match msg {
            Msg::StyledSelectChanged(field_id, ..) => field_id,
            _ => return,
        };
        if &self.field_id != field_id {
            return;
        }
        match msg {
            Msg::StyledSelectChanged(_, StyledSelectChanged::DropDownVisibility(b)) => {
                self.opened = *b;
                if !self.opened {
                    self.text_filter.clear();
                }
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::Text(text)) => {
                orders.skip();
                self.text_filter = text.clone();
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::Changed(Some(v))) => {
                self.values.clear();
                self.values.push(*v);
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::Changed(None)) => {
                self.values.clear();
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::RemoveMulti(v)) => {
                let len = self.values.len();
                let old = std::mem::replace(&mut self.values, Vec::with_capacity(len));

                for u in old {
                    if u != *v {
                        self.values.push(u);
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectOption<'l>>,
{
    pub id: FieldId,
    pub variant: SelectVariant,
    pub dropdown_width: Option<usize>,
    pub name: &'l str,
    pub valid: bool,
    pub is_multi: bool,
    pub options: Option<Options>,
    pub selected: Vec<StyledSelectOption<'l>>,
    pub text_filter: &'l str,
    pub opened: bool,
    pub clearable: bool,
}

impl<'l, Options> Default for StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectOption<'l>>,
{
    fn default() -> Self {
        Self {
            id: FieldId::TextFilterBoard,
            variant: Default::default(),
            dropdown_width: None,
            name: "",
            valid: true,
            is_multi: false,
            options: None,
            selected: vec![],
            text_filter: "",
            opened: false,
            clearable: false,
        }
    }
}

impl<'l, Options> StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectOption<'l>>,
{
    pub fn render(self) -> Node<Msg> {
        let StyledSelect {
            id,
            variant,
            dropdown_width,
            name,
            valid,
            is_multi,
            options,
            selected,
            text_filter,
            opened,
            clearable,
        } = self;

        let on_text = super::events::on_keyup_change_select_text(id.clone());

        let on_handler =
            super::events::on_click_change_select_dropdown_visibility(id.clone(), opened);

        let dropdown_style = dropdown_width.map_or_else(
            || "width: 100%;".to_string(),
            |n| format!("width: {}px;", n),
        );

        let action_icon = if clearable && !selected.is_empty() {
            let on_click = super::events::on_click_change_select_selected(id.clone(), None);
            StyledIcon {
                icon: Icon::Close,
                class_list: "chevronIcon",
                on_click: Some(on_click),
                ..Default::default()
            }
            .render()
        } else if (selected.is_empty() || !is_multi) && variant != SelectVariant::Empty {
            StyledIcon {
                icon: Icon::ChevronDown,
                class_list: "chevronIcon",
                ..Default::default()
            }
            .render()
        } else {
            empty![]
        };

        let skip = {
            let len = selected.len();
            selected
                .iter()
                .fold(HashMap::with_capacity(len), |mut h, o| {
                    h.insert(o.value, true);
                    h
                })
        };
        let children: Vec<Node<Msg>> = if let Some(options) = options {
            options
                .filter(|o| !skip.contains_key(&o.value) && o.match_text(text_filter))
                .map(|child| {
                    let on_change = super::events::on_click_change_select_selected(
                        id.clone(),
                        Some(child.value()),
                    );
                    let node = child.render_option();
                    div![C!["option"], on_change, on_handler.clone(), node]
                })
                .collect()
        } else {
            vec![]
        };

        seed::div![
            C!["styledSelect", variant.to_str(), IF![!valid => "invalid"]],
            attrs![At::Style => dropdown_style.as_str()],
            super::events::on_keyup_noop(),
            div![
                C!["valueContainer", variant.to_str()],
                on_handler,
                match is_multi {
                    true => vec![div![
                        C!["valueMulti"],
                        selected
                            .into_iter()
                            .map(|m| Self::multi_value(m, id.clone()))
                            .collect::<Vec<Node<Msg>>>(),
                        IF![children.is_empty() => div![C!["placeholder"], "Select"]],
                        IF![!children.is_empty() => div![C!["addMore"], StyledIcon::from(Icon::Plus).render(), "Add more"]],
                    ]],
                    false => selected
                        .into_iter()
                        .map(|m| m.render_value())
                        .collect::<Vec<Node<Msg>>>(),
                },
                action_icon,
            ],
            div![
                C!["dropDown"],
                attrs![At::Style => dropdown_style.as_str()],
                IF![opened => seed::input![
                    C!["dropDownInput"],
                    attrs![
                        At::Name => name,
                        At::Type => "text"
                        At::Placeholder => "Search"
                        At::AutoFocus => "true",
                    ],
                    on_text,
                ]],
                match (opened, children.is_empty()) {
                    (false, _) => empty![],
                    (_, true) => seed::div![C!["noOptions"], "No results"],
                    _ => seed::div![C!["options"], children],
                }
            ]
        ]
    }

    fn multi_value(child: StyledSelectOption, id: FieldId) -> Node<Msg> {
        let handler = super::events::on_click_change_select_remove_multi(id, child.value());

        div![C!["valueMultiItem"], child.render_multi_value(), handler]
    }
}
