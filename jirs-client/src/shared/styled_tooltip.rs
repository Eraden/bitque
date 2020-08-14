use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug, Copy, Clone)]
pub enum Variant {
    About,
    Messages,
    TableBuilder,
    CodeBuilder,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Messages
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::About => f.write_str("about"),
            Variant::Messages => f.write_str("messages"),
            Variant::TableBuilder => f.write_str("tableTooltip"),
            Variant::CodeBuilder => f.write_str("codeTooltip"),
        }
    }
}

pub struct StyledTooltip {
    visible: bool,
    class_name: String,
    children: Vec<Node<Msg>>,
    variant: Variant,
}

impl ToNode for StyledTooltip {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledTooltip {
    pub fn build() -> StyledTooltipBuilder {
        StyledTooltipBuilder::default()
    }
}

#[derive(Default)]
pub struct StyledTooltipBuilder {
    visible: bool,
    class_list: Vec<String>,
    children: Vec<Node<Msg>>,
    variant: Variant,
}

impl StyledTooltipBuilder {
    pub fn visible(mut self, b: bool) -> Self {
        self.visible = b;
        self
    }

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn add_child(mut self, child: Node<Msg>) -> Self {
        self.children.push(child);
        self
    }

    pub fn about_tooltip(mut self) -> Self {
        self.variant = Variant::About;
        self
    }

    pub fn messages_tooltip(mut self) -> Self {
        self.variant = Variant::Messages;
        self
    }

    pub fn table_tooltip(mut self) -> Self {
        self.variant = Variant::TableBuilder;
        self
    }

    pub fn code_tooltip(mut self) -> Self {
        self.variant = Variant::CodeBuilder;
        self
    }

    pub fn build(self) -> StyledTooltip {
        StyledTooltip {
            visible: self.visible,
            class_name: self.class_list.join(" "),
            children: self.children,
            variant: self.variant,
        }
    }
}

pub fn render(values: StyledTooltip) -> Node<Msg> {
    let StyledTooltip {
        visible,
        class_name,
        children,
        variant,
    } = values;
    if visible {
        div![
            attrs![At::Class => format!("styledTooltip {} {}", class_name, variant)],
            children
        ]
    } else {
        empty!()
    }
}
