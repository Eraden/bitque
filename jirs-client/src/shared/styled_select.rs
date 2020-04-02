use seed::{prelude::*, *};

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
    pub id: FieldId,
    pub variant: Variant,
    pub dropdown_width: Option<usize>,
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub valid: bool,
    pub is_multi: bool,
    pub allow_clear: bool,
    pub options: Vec<Child>,
    pub selected: Vec<Child>,
    pub text_filter: String,
    pub opened: bool,
}

impl<Child> ToNode for StyledSelect<Child>
where
    Child: SelectOption + PartialEq,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
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
        (true, true) => crate::shared::styled_icon(crate::model::Icon::Close),
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

fn render_option(
    content: Node<Msg>,
    on_change: EventHandler<Msg>,
    on_click: EventHandler<Msg>,
) -> Node<Msg> {
    div![attrs![At::Class => "option"], on_change, on_click, content]
}

fn render_value(mut content: Node<Msg>) -> Node<Msg> {
    content.add_class("value");
    content
}
