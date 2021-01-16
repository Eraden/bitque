use {
    crate::{
        components::{
            styled_icon::{Icon, StyledIcon},
            styled_select_child::*,
        },
        shared::ToNode,
        FieldId, Msg,
    },
    seed::{prelude::*, *},
};

// pub trait ChildIter<'l> = Iterator<Item = StyledSelectChildBuilder<'l>>;

#[derive(Clone, Debug, PartialEq)]
pub enum StyledSelectChanged {
    Text(String),
    DropDownVisibility(bool),
    Changed(Option<u32>),
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

impl Variant {
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            Variant::Empty => "empty",
            Variant::Normal => "normal",
        }
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
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
                self.text_filter = text.clone();
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::Changed(Some(v))) => {
                self.values = vec![*v];
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::Changed(None)) => {
                self.values.clear();
            }
            Msg::StyledSelectChanged(_, StyledSelectChanged::RemoveMulti(v)) => {
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

pub struct StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
    id: FieldId,
    variant: Variant,
    dropdown_width: Option<usize>,
    name: Option<&'l str>,
    valid: bool,
    is_multi: bool,
    options: Option<Options>,
    selected: Vec<StyledSelectChildBuilder<'l>>,
    text_filter: &'l str,
    opened: bool,
    clearable: bool,
}

impl<'l, Options> ToNode for StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl<'l, Options> StyledSelect<'l, Options>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
    pub fn build() -> StyledSelectBuilder<'l, Options> {
        StyledSelectBuilder {
            variant: None,
            dropdown_width: None,
            name: None,
            valid: None,
            is_multi: None,
            options: None,
            selected: None,
            text_filter: None,
            opened: None,
            clearable: false,
        }
    }
}

#[derive(Debug)]
pub struct StyledSelectBuilder<'l, Options>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
    variant: Option<Variant>,
    dropdown_width: Option<usize>,
    name: Option<&'l str>,
    valid: Option<bool>,
    is_multi: Option<bool>,
    options: Option<Options>,
    selected: Option<Vec<StyledSelectChildBuilder<'l>>>,
    text_filter: Option<&'l str>,
    opened: Option<bool>,
    clearable: bool,
}

impl<'l, Options> StyledSelectBuilder<'l, Options>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
    pub fn build(self, id: FieldId) -> StyledSelect<'l, Options> {
        StyledSelect {
            id,
            variant: self.variant.unwrap_or_default(),
            dropdown_width: self.dropdown_width,
            name: self.name,
            valid: self.valid.unwrap_or(true),
            is_multi: self.is_multi.unwrap_or_default(),
            options: self.options,
            selected: self.selected.unwrap_or_default(),
            text_filter: self.text_filter.unwrap_or_default(),
            opened: self.opened.unwrap_or_default(),
            clearable: self.clearable,
        }
    }

    pub fn state<'state: 'l>(self, state: &'state StyledSelectState) -> Self {
        self.opened(state.opened)
            .text_filter(state.text_filter.as_str())
    }

    pub fn dropdown_width(mut self, dropdown_width: usize) -> Self {
        self.dropdown_width = Some(dropdown_width);
        self
    }

    pub fn name(mut self, name: &'l str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn text_filter(mut self, text_filter: &'l str) -> Self {
        self.text_filter = Some(text_filter);
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

    pub fn options(mut self, options: Options) -> Self {
        self.options = Some(options);
        self
    }

    pub fn selected(mut self, selected: Vec<StyledSelectChildBuilder<'l>>) -> Self {
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

    pub fn clearable(mut self) -> Self {
        self.clearable = true;
        self
    }
}

pub fn render<'l, Options>(values: StyledSelect<'l, Options>) -> Node<Msg>
where
    Options: Iterator<Item = StyledSelectChildBuilder<'l>>,
{
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
    } = values;

    let on_text = {
        let field_id = id.clone();
        input_ev(Ev::KeyUp, move |value| {
            Msg::StyledSelectChanged(field_id, StyledSelectChanged::Text(value))
        })
    };

    let on_handler = {
        let field_id = id.clone();
        mouse_ev(Ev::Click, move |_| {
            Msg::StyledSelectChanged(field_id, StyledSelectChanged::DropDownVisibility(!opened))
        })
    };

    let dropdown_style = dropdown_width
        .map(|n| format!("width: {}px;", n))
        .unwrap_or_else(|| "width: 100%;".to_string());

    let mut select_class = vec!["styledSelect".to_string(), format!("{}", variant)];
    if !valid {
        select_class.push("invalid".to_string());
    }

    let action_icon = if clearable && !selected.is_empty() {
        let on_click = {
            let field_id = id.clone();
            mouse_ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Msg::StyledSelectChanged(field_id, StyledSelectChanged::Changed(None))
            })
        };
        StyledIcon::build(Icon::Close)
            .add_class("chevronIcon")
            .on_click(on_click)
            .build()
            .into_node()
    } else if (selected.is_empty() || !is_multi) && variant != Variant::Empty {
        StyledIcon::build(Icon::ChevronDown)
            .add_class("chevronIcon")
            .build()
            .into_node()
    } else {
        empty![]
    };

    let children: Vec<Node<Msg>> = if let Some(options) = options {
        options
            .filter(|o| !selected.contains(&o) && o.match_text(text_filter))
            .map(|child| {
                let child = child.build(DisplayType::SelectOption);
                let value = child.value();
                let node = child.into_node();

                let on_change = {
                    let field_id = id.clone();
                    mouse_ev(Ev::Click, move |_| {
                        Msg::StyledSelectChanged(
                            field_id,
                            StyledSelectChanged::Changed(Some(value)),
                        )
                    })
                };
                div![
                    attrs![At::Class => "option"],
                    on_change,
                    on_handler.clone(),
                    node
                ]
            })
            .collect()
    } else {
        vec![]
    };

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
            on_handler,
            value,
            action_icon,
        ],
        div![
            C!["dropDown"],
            attrs![At::Style => dropdown_style.as_str()],
            text_input,
            option_list
        ]
    ]
}

fn render_value(mut content: Node<Msg>) -> Node<Msg> {
    content.add_class("value");
    content
}

fn into_multi_value(opt: StyledSelectChildBuilder, id: FieldId) -> Node<Msg> {
    let close_icon = StyledIcon::build(Icon::Close).size(14).build().into_node();
    let child = opt.build(DisplayType::SelectValue);
    let value = child.value();

    let mut opt = child.into_node();
    opt.add_class("value");
    opt.add_child(close_icon);

    let handler = {
        let field_id = id.clone();
        mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            Msg::StyledSelectChanged(field_id, StyledSelectChanged::RemoveMulti(value))
        })
    };

    div![attrs![At::Class => "valueMultiItem"], opt, handler]
}
