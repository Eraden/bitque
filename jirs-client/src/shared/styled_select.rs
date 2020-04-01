use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Variant {
    Empty,
    Normal,
}

pub struct StyledSelect<Child>
where
    Child: ToNode,
{
    pub on_change: EventHandler<Msg>,
    pub variant: Variant,
    pub width: usize,
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub valid: bool,
    pub is_multi: bool,
    pub allow_clear: bool,
    pub options: Vec<Child>,
}

impl<Child> ToNode for StyledSelect<Child>
where
    Child: ToNode,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render<Child>(values: StyledSelect<Child>) -> Node<Msg>
where
    Child: ToNode,
{
    let StyledSelect {
        on_change,
        variant,
        width,
        name,
        placeholder,
        valid,
        is_multi,
        allow_clear,
        options,
    } = values;

    let select_style = format!("width: {width}px", width = width);
    let mut select_class = vec!["styledSelect"];
    if !valid {
        select_class.push("invalid");
    }

    let children: Vec<Node<Msg>> = options
        .into_iter()
        .map(|child| render_option(child.into_node()))
        .collect();

    let clear_icon = match allow_clear {
        true => crate::shared::styled_icon(crate::model::Icon::Close),
        false => empty![],
    };

    seed::div![
        on_change.clone(),
        attrs![At::Class => "styledSelect", At::Style => select_style],
        seed::input![
            attrs![
                At::Class => "dropDownInput",
                At::Type => "text"
                At::Placeholder => "Search"
                At::AutoFocus => true,
            ],
            on_change,
        ],
        clear_icon,
        seed::div![
            attrs![
                At::Class => "options",
            ],
            children
        ],
        seed::div![attrs![At::Class => "noOptions"], "No results"]
    ]
}

pub fn render_option(content: Node<Msg>) -> Node<Msg> {
    seed::div![attrs![At::Class => "option"], content,]
}
