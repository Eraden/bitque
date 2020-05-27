use seed::{prelude::*, *};

use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_select_child::*;
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Clone, Debug, PartialEq)]
pub enum StyledSelectChange {
    Text(String),
    DropDownVisibility(bool),
    Changed(u32),
    RemoveMulti(u32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Variant {
    Empty,
    Normal,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Empty
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Empty => f.write_str("empty"),
            Variant::Normal => f.write_str("normal"),
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

    pub fn update(&mut self, msg: &Msg, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::StyledSelectChanged(field_id, StyledSelectChange::DropDownVisibility(b))
                if *field_id == self.field_id =>
            {
                self.opened = *b;
                if !self.opened {
                    self.text_filter.clear();
                }
            }
            Msg::StyledSelectChanged(field_id, StyledSelectChange::Text(text))
                if *field_id == self.field_id =>
            {
                self.text_filter = text.clone();
            }
            Msg::StyledSelectChanged(field_id, StyledSelectChange::Changed(v))
                if field_id == &self.field_id =>
            {
                self.values = vec![*v];
            }
            Msg::StyledSelectChanged(field_id, StyledSelectChange::RemoveMulti(v))
                if field_id == &self.field_id =>
            {
                let mut old = vec![];
                std::mem::swap(&mut old, &mut self.values);

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

pub struct StyledSelect {
    id: FieldId,
    variant: Variant,
    dropdown_width: Option<usize>,
    name: Option<String>,
    valid: bool,
    is_multi: bool,
    allow_clear: bool,
    options: Vec<StyledSelectChildBuilder>,
    selected: Vec<StyledSelectChildBuilder>,
    text_filter: String,
    opened: bool,
}

impl ToNode for StyledSelect {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledSelect {
    pub fn build(id: FieldId) -> StyledSelectBuilder {
        StyledSelectBuilder {
            id,
            variant: None,
            dropdown_width: None,
            name: None,
            valid: None,
            is_multi: None,
            allow_clear: None,
            options: None,
            selected: None,
            text_filter: None,
            opened: None,
        }
    }
}

#[derive(Debug)]
pub struct StyledSelectBuilder {
    id: FieldId,
    variant: Option<Variant>,
    dropdown_width: Option<usize>,
    name: Option<String>,
    valid: Option<bool>,
    is_multi: Option<bool>,
    allow_clear: Option<bool>,
    options: Option<Vec<StyledSelectChildBuilder>>,
    selected: Option<Vec<StyledSelectChildBuilder>>,
    text_filter: Option<String>,
    opened: Option<bool>,
}

impl StyledSelectBuilder {
    pub fn build(self) -> StyledSelect {
        StyledSelect {
            id: self.id,
            variant: self.variant.unwrap_or_default(),
            dropdown_width: self.dropdown_width,
            name: self.name,
            valid: self.valid.unwrap_or(true),
            is_multi: self.is_multi.unwrap_or_default(),
            allow_clear: self.allow_clear.unwrap_or_default(),
            options: self.options.unwrap_or_default(),
            selected: self.selected.unwrap_or_default(),
            text_filter: self.text_filter.unwrap_or_default(),
            opened: self.opened.unwrap_or_default(),
        }
    }

    pub fn state(self, state: &StyledSelectState) -> Self {
        self.opened(state.opened)
            .text_filter(state.text_filter.as_str())
    }

    pub fn dropdown_width(mut self, dropdown_width: usize) -> Self {
        self.dropdown_width = Some(dropdown_width);
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn text_filter<S>(mut self, text_filter: S) -> Self
    where
        S: Into<String>,
    {
        self.text_filter = Some(text_filter.into());
        self
    }

    pub fn opened(mut self, opened: bool) -> Self {
        self.opened = Some(opened);
        self
    }

    pub fn valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }

    pub fn options(mut self, options: Vec<StyledSelectChildBuilder>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn selected(mut self, selected: Vec<StyledSelectChildBuilder>) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn normal(mut self) -> Self {
        self.variant = Some(Variant::Normal);
        self
    }

    pub fn empty(mut self) -> Self {
        self.variant = Some(Variant::Empty);
        self
    }

    pub fn multi(mut self) -> Self {
        self.is_multi = Some(true);
        self
    }
}

pub fn render(values: StyledSelect) -> Node<Msg> {
    let StyledSelect {
        id,
        variant,
        dropdown_width,
        name,
        valid,
        is_multi,
        allow_clear,
        options,
        selected,
        text_filter,
        opened,
    } = values;

    let field_id = id.clone();
    let on_text = input_ev(Ev::KeyUp, move |value| {
        Msg::StyledSelectChanged(field_id, StyledSelectChange::Text(value))
    });

    let field_id = id.clone();
    let visibility_handler = mouse_ev(Ev::Click, move |_| {
        Msg::StyledSelectChanged(field_id, StyledSelectChange::DropDownVisibility(!opened))
    });

    let dropdown_style = dropdown_width
        .map(|n| format!("width: {}px;", n))
        .unwrap_or_else(|| "width: 100%;".to_string());

    let mut select_class = vec!["styledSelect".to_string(), format!("{}", variant)];
    if !valid {
        select_class.push("invalid".to_string());
    }

    let chevron_down = if (selected.is_empty() || !is_multi) && variant != Variant::Empty {
        StyledIcon::build(Icon::ChevronDown)
            .add_class("chevronIcon")
            .build()
            .into_node()
    } else {
        empty![]
    };

    let children: Vec<Node<Msg>> = options
        .into_iter()
        .filter(|o| !selected.contains(&o) && o.match_text(text_filter.as_str()))
        .map(|child| {
            let child = child.build(DisplayType::SelectOption);
            let value = child.value();
            let node = child.into_node();
            let field_id = id.clone();
            let on_change = mouse_ev(Ev::Click, move |_| {
                Msg::StyledSelectChanged(field_id, StyledSelectChange::Changed(value))
            });
            div![
                attrs![At::Class => "option"],
                on_change,
                visibility_handler.clone(),
                node
            ]
        })
        .collect();

    let text_input = if opened {
        seed::input![
            attrs![
                At::Name => name.unwrap_or_default(),
                At::Class => "dropDownInput",
                At::Type => "text"
                At::Placeholder => "Search"
                At::AutoFocus => "true",
            ],
            on_text,
        ]
    } else {
        empty![]
    };

    let clear_icon = match (opened, allow_clear) {
        (true, true) => StyledIcon::build(Icon::Close).build().into_node(),
        _ => empty![],
    };

    let option_list = match (opened, children.is_empty()) {
        (false, _) => empty![],
        (_, true) => seed::div![attrs![At::Class => "noOptions"], "No results"],
        _ => seed::div![attrs![ At::Class => "options" ], children],
    };

    let value: Vec<Node<Msg>> = if is_multi {
        let add_icon = StyledIcon::build(Icon::Plus).build().into_node();
        let mut children: Vec<Node<Msg>> = selected
            .into_iter()
            .map(|m| into_multi_value(m, id.clone()))
            .collect();

        if !children.is_empty() {
            children.push(div![attrs![At::Class => "addMore"], add_icon, "Add more"]);
        } else {
            children.push(div![attrs![At::Class => "placeholder"], "Select"]);
        }

        vec![div![attrs![At::Class => "valueMulti"], children]]
    } else {
        selected
            .into_iter()
            .map(|m| render_value(m.build(DisplayType::SelectValue).into_node()))
            .collect()
    };

    seed::div![
        attrs![At::Class => select_class.join(" "), At::Style => dropdown_style.as_str()],
        keyboard_ev(Ev::KeyUp, |ev| {
            ev.stop_propagation();
            None as Option<Msg>
        }),
        div![
            attrs![At::Class => format!("valueContainer {}", variant)],
            visibility_handler,
            value,
            chevron_down,
        ],
        div![
            class!["dropDown"],
            attrs![At::Style => dropdown_style.as_str()],
            text_input,
            clear_icon,
            option_list
        ]
    ]
}

fn render_value(mut content: Node<Msg>) -> Node<Msg> {
    content.add_class("value");
    content
}

fn into_multi_value(opt: StyledSelectChildBuilder, field_id: FieldId) -> Node<Msg> {
    let close_icon = StyledIcon::build(Icon::Close).size(14).build().into_node();
    let child = opt.build(DisplayType::SelectValue);
    let value = child.value();

    let mut opt = child.into_node();
    opt.add_class("value");
    opt.add_child(close_icon);

    let handler = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::StyledSelectChanged(field_id, StyledSelectChange::RemoveMulti(value))
    });

    div![attrs![At::Class => "valueMultiItem"], opt, handler]
}
