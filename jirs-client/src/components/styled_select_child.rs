use seed::prelude::*;
use seed::*;

use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select::SelectVariant;
use crate::Msg;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum DisplayType {
    SelectOption,
    SelectValue,
    SelectMultiValue,
}

pub struct StyledSelectOption<'l> {
    pub name: Option<&'l str>,
    pub icon: Option<Node<Msg>>,
    pub text: Option<&'l str>,
    pub value: u32,
    pub class_list: &'l str,
    pub variant: SelectVariant,
}

impl<'l> Default for StyledSelectOption<'l> {
    fn default() -> Self {
        Self {
            name: None,
            icon: None,
            text: None,
            value: 0,
            class_list: "",
            variant: Default::default(),
        }
    }
}

impl<'l> StyledSelectOption<'l> {
    #[inline(always)]
    pub fn value(&self) -> u32 {
        self.value
    }

    #[inline(always)]
    pub fn render_value(self) -> Node<Msg> {
        self.render(DisplayType::SelectValue)
    }

    #[inline(always)]
    pub fn render_multi_value(self) -> Node<Msg> {
        self.render(DisplayType::SelectMultiValue)
    }

    #[inline(always)]
    pub fn render_option(self) -> Node<Msg> {
        self.render(DisplayType::SelectOption)
    }

    #[inline(always)]
    pub fn match_text(&self, text: &str) -> bool {
        self.text
            .as_ref()
            .map(|t| t.to_lowercase().contains(text.to_lowercase().as_str()))
            .unwrap_or(true)
    }

    #[inline(always)]
    pub fn render(self, display_type: DisplayType) -> Node<Msg> {
        let StyledSelectOption {
            name,
            icon,
            text,
            value: _,
            class_list,
            variant,
        } = self;

        let label_node = text.map_or_else(
            || Node::Empty,
            |text| {
                div![
                    C![
                        variant.to_str(),
                        name.as_deref()
                            .map(|s| format!("{}Label", s))
                            .unwrap_or_default(),
                        match display_type {
                            DisplayType::SelectOption => "optionLabel",
                            DisplayType::SelectValue | DisplayType::SelectMultiValue =>
                                "selectItemLabel",
                        },
                        class_list
                    ],
                    text
                ]
            },
        );

        div![
            C![
                variant.to_str(),
                name.as_deref().unwrap_or_default(),
                match display_type {
                    DisplayType::SelectOption => "optionItem",
                    DisplayType::SelectValue | DisplayType::SelectMultiValue => "selectItem value",
                },
                class_list
            ],
            icon,
            label_node,
            IF![display_type == DisplayType::SelectMultiValue => close_icon()]
        ]
    }
}

#[inline(always)]
fn close_icon() -> Node<Msg> {
    StyledIcon {
        icon: Icon::Close,
        size: Some(14),
        ..Default::default()
    }
    .render()
}
