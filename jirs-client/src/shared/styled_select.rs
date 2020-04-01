use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

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
}

pub struct StyledSelect<Child>
where
    Child: SelectOption + PartialEq,
{
    pub on_change: EventHandler<Msg>,
    pub variant: Variant,
    pub dropdown_width: Option<usize>,
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub valid: bool,
    pub is_multi: bool,
    pub allow_clear: bool,
    pub options: Vec<Child>,
    pub selected: Option<Child>,
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
        on_change,
        variant,
        dropdown_width,
        name,
        placeholder: _,
        valid,
        is_multi: _,
        allow_clear,
        options,
        selected,
    } = values;

    let dropdown_style = dropdown_width
        .map(|n| format!("width: {}px", n))
        .unwrap_or_default();
    let mut select_class = vec!["styledSelect".to_string(), format!("{}", variant)];
    if !valid {
        select_class.push("invalid".to_string());
    }

    let children: Vec<Node<Msg>> = options
        .into_iter()
        .filter(|o| Some(o) != selected.as_ref())
        .map(|child| render_option(child.into_option()))
        .collect();

    let value = selected
        .map(|m| render_value(m.into_value()))
        .unwrap_or_else(|| empty![]);

    let clear_icon = match allow_clear {
        true => crate::shared::styled_icon(crate::model::Icon::Close),
        false => empty![],
    };

    let option_list = if children.is_empty() {
        seed::div![attrs![At::Class => "noOptions"], "No results"]
    } else {
        seed::div![
            attrs![
                At::Class => "options",
            ],
            children
        ]
    };

    seed::div![
        on_change.clone(),
        attrs![At::Class => select_class.join(" ")],
        div![
            attrs![At::Class => format!("valueContainer {}", variant)],
            value
        ],
        div![
            attrs![At::Class => "dropDown", At::Style => dropdown_style],
            seed::input![
                attrs![
                    At::Name => name.unwrap_or_default(),
                    At::Class => "dropDownInput",
                    At::Type => "text"
                    At::Placeholder => "Search"
                    At::AutoFocus => true,
                ],
                on_change,
            ],
            clear_icon,
            option_list
        ]
    ]
}

fn render_option(content: Node<Msg>) -> Node<Msg> {
    div![attrs![At::Class => "option"], content]
}

fn render_value(mut content: Node<Msg>) -> Node<Msg> {
    content.add_class("value");
    content
}
