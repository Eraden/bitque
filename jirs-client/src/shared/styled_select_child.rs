use seed::{prelude::*, *};

use crate::shared::styled_select::Variant;
use crate::shared::ToNode;
use crate::Msg;

pub trait ToStyledSelectChild {
    fn to_select_child(&self) -> StyledSelectChildBuilder;
}

pub enum DisplayType {
    SelectOption,
    SelectValue,
}

pub struct StyledSelectChild {
    name: Option<String>,
    icon: Option<Node<Msg>>,
    text: Option<String>,
    display_type: DisplayType,
    value: u32,
    class_list: Vec<String>,
    variant: Variant,
}

impl StyledSelectChild {
    pub fn build() -> StyledSelectChildBuilder {
        StyledSelectChildBuilder {
            icon: None,
            text: None,
            name: None,
            value: 0,
            class_list: vec![],
            variant: Default::default(),
        }
    }

    #[inline]
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl ToNode for StyledSelectChild {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Debug)]
pub struct StyledSelectChildBuilder {
    icon: Option<Node<Msg>>,
    text: Option<String>,
    name: Option<String>,
    value: u32,
    class_list: Vec<String>,
    variant: Variant,
}

impl PartialEq for StyledSelectChildBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl StyledSelectChildBuilder {
    pub fn icon(mut self, icon: Node<Msg>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.text = Some(text.into());
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn match_text(&self, text: &str) -> bool {
        self.text
            .as_ref()
            .map(|t| t.contains(text))
            .unwrap_or_default()
    }

    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn build(self, display_type: DisplayType) -> StyledSelectChild {
        StyledSelectChild {
            name: self.name,
            icon: self.icon,
            text: self.text,
            display_type,
            value: self.value,
            class_list: self.class_list,
            variant: self.variant,
        }
    }
}

pub fn render(values: StyledSelectChild) -> Node<Msg> {
    let StyledSelectChild {
        name,
        icon,
        text,
        display_type,
        value: _,
        mut class_list,
        variant,
    } = values;

    class_list.push(format!("{}", variant));

    let label_class = match display_type {
        DisplayType::SelectOption => vec![
            "optionLabel".to_string(),
            variant.to_string(),
            name.as_ref().cloned().unwrap_or_default(),
            name.as_ref()
                .map(|s| format!("{}Label", s))
                .unwrap_or_default(),
            class_list.join(" "),
        ],
        DisplayType::SelectValue => vec![
            "selectItemLabel".to_string(),
            variant.to_string(),
            name.as_ref().cloned().unwrap_or_default(),
            name.as_ref()
                .map(|s| format!("{}Label", s))
                .unwrap_or_default(),
            class_list.join(" "),
        ],
    }
    .join(" ");

    let wrapper_class = match display_type {
        DisplayType::SelectOption => vec![
            "optionItem".to_string(),
            name.as_ref().cloned().unwrap_or_default(),
            class_list.join(" "),
        ],
        DisplayType::SelectValue => vec![
            "selectItem".to_string(),
            name.as_ref().cloned().unwrap_or_default(),
            class_list.join(" "),
        ],
    }
    .join(" ");

    let icon_node = match icon {
        Some(icon) => icon,
        _ => empty![],
    };

    let label_node = match text {
        Some(text) => div![class![label_class.as_str()], text],
        _ => empty![],
    };

    div![class![wrapper_class.as_str()], icon_node, label_node]
}

impl ToStyledSelectChild for jirs_data::User {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let avatar = crate::shared::styled_avatar::StyledAvatar::build()
            .avatar_url(self.avatar_url.as_ref().cloned().unwrap_or_default())
            .size(20)
            .name(self.name.as_str())
            .build()
            .into_node();
        StyledSelectChild::build()
            .value(self.id as u32)
            .icon(avatar)
            .text(self.name.as_str())
    }
}

impl ToStyledSelectChild for jirs_data::IssuePriority {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let icon = crate::shared::styled_icon::StyledIcon::build(self.clone().into())
            .add_class(self.to_string())
            .build()
            .into_node();
        let text = self.to_string();

        StyledSelectChild::build()
            .icon(icon)
            .value(self.clone().into())
            .text(text)
            .add_class(format!("{}", self))
    }
}

impl ToStyledSelectChild for jirs_data::IssueStatus {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let text = self.to_label();

        StyledSelectChild::build()
            .value(self.clone().into())
            .add_class(text)
            .text(text)
    }
}

impl ToStyledSelectChild for jirs_data::IssueType {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let name = self.to_label().to_owned();

        let type_icon = crate::shared::styled_icon::StyledIcon::build(self.clone().into())
            .add_class(name.as_str())
            .build()
            .into_node();

        StyledSelectChild::build()
            .add_class(name.as_str())
            .text(name)
            .icon(type_icon)
            .value(self.clone().into())
    }
}

impl ToStyledSelectChild for jirs_data::ProjectCategory {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let name = self.to_string();

        StyledSelectChild::build()
            .add_class(name.as_str())
            .text(name)
            .value(self.clone().into())
    }
}

impl ToStyledSelectChild for jirs_data::UserRole {
    fn to_select_child(&self) -> StyledSelectChildBuilder {
        let name = self.to_string();

        StyledSelectChild::build()
            .add_class(name.as_str())
            .add_class("capitalize")
            .text(name)
            .value(self.clone().into())
    }
}
