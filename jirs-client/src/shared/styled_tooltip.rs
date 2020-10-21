use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug, Copy, Clone)]
pub enum Variant {
    About,
    Messages,
    TableBuilder,
    CodeBuilder,
    DateTimeBuilder,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Messages
    }
}

impl Variant {
    pub fn to_str(&self) -> &'static str {
        match self {
            Variant::About => "about",
            Variant::Messages => "messages",
            Variant::TableBuilder => "tableTooltip",
            Variant::CodeBuilder => "codeTooltip",
            Variant::DateTimeBuilder => "dateTimeTooltip",
        }
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

pub struct StyledTooltip<'l> {
    visible: bool,
    class_list: Vec<&'l str>,
    children: Vec<Node<Msg>>,
    variant: Variant,
}

impl<'l> ToNode for StyledTooltip<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl<'l> StyledTooltip<'l> {
    pub fn build() -> StyledTooltipBuilder<'l> {
        StyledTooltipBuilder::default()
    }
}

#[derive(Default)]
pub struct StyledTooltipBuilder<'l> {
    visible: bool,
    class_list: Vec<&'l str>,
    children: Vec<Node<Msg>>,
    variant: Variant,
}

impl<'l> StyledTooltipBuilder<'l> {
    pub fn visible(mut self, b: bool) -> Self {
        self.visible = b;
        self
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
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

    // pub fn table_tooltip(mut self) -> Self {
    //     self.variant = Variant::TableBuilder;
    //     self
    // }
    //
    // pub fn code_tooltip(mut self) -> Self {
    //     self.variant = Variant::CodeBuilder;
    //     self
    // }

    pub fn date_time_picker(mut self) -> Self {
        self.variant = Variant::DateTimeBuilder;
        self
    }

    pub fn build(self) -> StyledTooltip<'l> {
        StyledTooltip {
            visible: self.visible,
            class_list: self.class_list,
            children: self.children,
            variant: self.variant,
        }
    }
}

pub fn render(values: StyledTooltip) -> Node<Msg> {
    let StyledTooltip {
        visible,
        class_list,
        children,
        variant,
    } = values;
    if visible {
        div![
            attrs![At::Class => format!("styledTooltip {} {}", class_list.join(" "), variant)],
            children
        ]
    } else {
        empty!()
    }
}
