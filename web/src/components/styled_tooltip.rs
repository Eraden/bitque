use seed::prelude::*;
use seed::*;

use crate::Msg;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum TooltipVariant {
    About,
    Messages,
    TableBuilder,
    CodeBuilder,
    DateTimeBuilder,
}

impl Default for TooltipVariant {
    fn default() -> Self {
        TooltipVariant::Messages
    }
}

impl TooltipVariant {
    pub fn to_str(self) -> &'static str {
        match self {
            TooltipVariant::About => "about",
            TooltipVariant::Messages => "messages",
            TooltipVariant::TableBuilder => "tableTooltip",
            TooltipVariant::CodeBuilder => "codeTooltip",
            TooltipVariant::DateTimeBuilder => "dateTimeTooltip",
        }
    }
}

impl std::fmt::Display for TooltipVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

pub struct StyledTooltip<'l> {
    pub visible: bool,
    pub class_list: &'l str,
    pub children: Vec<Node<Msg>>,
    pub variant: TooltipVariant,
}

impl<'l> StyledTooltip<'l> {
    pub fn render(self) -> Node<Msg> {
        let StyledTooltip {
            visible,
            class_list,
            children,
            variant,
        } = self;
        if visible {
            div![C!["styledTooltip", class_list, variant.to_str()], children]
        } else {
            empty!()
        }
    }
}
