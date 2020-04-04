use seed::{prelude::*, *};

use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Clone, Debug, PartialEq)]
pub enum StyledSelectChange {
    Text(String),
    DropDownVisibility(bool),
    Changed(u32),
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

pub trait SelectOption {
    fn into_option(self) -> Node<Msg>;

    fn into_value(self) -> Node<Msg>;

    fn match_text_filter(&self, text_filter: &str) -> bool;

    fn to_value(&self) -> u32;
}

pub struct StyledSelect<Child>
where
    Child: SelectOption + PartialEq,
{
    id: FieldId,
    variant: Variant,
    dropdown_width: Option<usize>,
    name: Option<String>,
    placeholder: Option<String>,
    valid: bool,
    is_multi: bool,
    allow_clear: bool,
    options: Vec<Child>,
    selected: Vec<Child>,
    text_filter: String,
    opened: bool,
}

impl<Child> ToNode for StyledSelect<Child>
where
    Child: SelectOption + PartialEq,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl<Child> StyledSelect<Child>
where
    Child: SelectOption + PartialEq,
{
    pub fn build(id: FieldId) -> StyledSelectBuilder<Child> {
        StyledSelectBuilder {
            id,
            variant: None,
            dropdown_width: None,
            name: None,
            placeholder: None,
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
pub struct StyledSelectBuilder<Child>
where
    Child: SelectOption + PartialEq,
{
    id: FieldId,
    variant: Option<Variant>,
    dropdown_width: Option<Option<usize>>,
    name: Option<Option<String>>,
    placeholder: Option<Option<String>>,
    valid: Option<bool>,
    is_multi: Option<bool>,
    allow_clear: Option<bool>,
    options: Option<Vec<Child>>,
    selected: Option<Vec<Child>>,
    text_filter: Option<String>,
    opened: Option<bool>,
}

impl<Child> StyledSelectBuilder<Child>
where
    Child: SelectOption + PartialEq,
{
    pub fn build(self) -> StyledSelect<Child> {
        StyledSelect {
            id: self.id,
            variant: self.variant.unwrap_or_default(),
            dropdown_width: self.dropdown_width.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            placeholder: self.placeholder.unwrap_or_default(),
            valid: self.valid.unwrap_or(true),
            is_multi: self.is_multi.unwrap_or_default(),
            allow_clear: self.allow_clear.unwrap_or_default(),
            options: self.options.unwrap_or_default(),
            selected: self.selected.unwrap_or_default(),
            text_filter: self.text_filter.unwrap_or_default(),
            opened: self.opened.unwrap_or_default(),
        }
    }

    pub fn dropdown_width(mut self, dropdown_width: usize) -> Self {
        self.dropdown_width = Some(Some(dropdown_width));
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = Some(Some(name.into()));
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

    pub fn options(mut self, options: Vec<Child>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn selected(mut self, selected: Vec<Child>) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn normal(mut self) -> Self {
        self.variant = Some(Variant::Normal);
        self
    }
}

pub fn render<Child>(values: StyledSelect<Child>) -> Node<Msg>
where
    Child: SelectOption + PartialEq,
{
    let StyledSelect {
        id,
        variant,
        dropdown_width,
        name,
        placeholder: _,
        valid,
        is_multi: _,
        allow_clear,
        options,
        selected,
        text_filter,
        opened,
    } = values;

    let on_text = input_ev(Ev::KeyUp, |value| {
        Msg::StyledSelectChanged(
            FieldId::IssueTypeEditModalTop,
            StyledSelectChange::Text(value),
        )
    });

    let field_id = id.clone();
    let visibility_handler = mouse_ev(Ev::Click, move |_| {
        Msg::StyledSelectChanged(field_id, StyledSelectChange::DropDownVisibility(!opened))
    });

    let dropdown_style = dropdown_width
        .map(|n| format!("width: {}px", n))
        .unwrap_or_default();
    let mut select_class = vec!["styledSelect".to_string(), format!("{}", variant)];
    if !valid {
        select_class.push("invalid".to_string());
    }

    let children: Vec<Node<Msg>> = options
        .into_iter()
        .filter(|o| !selected.contains(&o) && o.match_text_filter(text_filter.as_str()))
        .map(|child| {
            let value = child.to_value();
            let field_id = id.clone();
            let on_change = mouse_ev(Ev::Click, move |_| {
                Msg::StyledSelectChanged(field_id, StyledSelectChange::Changed(value))
            });
            div![
                attrs![At::Class => "option"],
                on_change,
                visibility_handler.clone(),
                child.into_option()
            ]
        })
        .collect();

    let value = selected.into_iter().map(|m| render_value(m.into_value()));

    let text_input = match opened {
        true => seed::input![
            attrs![
                At::Name => name.unwrap_or_default(),
                At::Class => "dropDownInput",
                At::Type => "text"
                At::Placeholder => "Search"
                At::AutoFocus => true,
            ],
            on_text.clone(),
        ],
        _ => empty![],
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

    seed::div![
        attrs![At::Class => select_class.join(" ")],
        div![
            attrs![At::Class => format!("valueContainer {}", variant)],
            visibility_handler,
            value,
        ],
        div![
            attrs![At::Class => "dropDown", At::Style => dropdown_style],
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
