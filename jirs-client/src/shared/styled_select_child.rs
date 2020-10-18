use std::borrow::Cow;

use seed::{prelude::*, *};

use crate::shared::styled_select::Variant;
use crate::shared::{ToChild, ToNode};
use crate::Msg;

pub enum DisplayType {
    SelectOption,
    SelectValue,
}

pub struct StyledSelectChild<'l> {
    name: Option<&'l str>,
    icon: Option<Node<Msg>>,
    text: Option<std::borrow::Cow<'l, str>>,
    display_type: DisplayType,
    value: u32,
    class_list: Vec<std::borrow::Cow<'l, str>>,
    variant: Variant,
}

impl<'l> StyledSelectChild<'l> {
    pub fn build() -> StyledSelectChildBuilder<'l> {
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

impl<'l> ToNode for StyledSelectChild<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Debug)]
pub struct StyledSelectChildBuilder<'l> {
    icon: Option<Node<Msg>>,
    text: Option<std::borrow::Cow<'l, str>>,
    name: Option<&'l str>,
    value: u32,
    class_list: Vec<std::borrow::Cow<'l, str>>,
    variant: Variant,
}

impl<'l> PartialEq for StyledSelectChildBuilder<'l> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'l> StyledSelectChildBuilder<'l> {
    pub fn icon(mut self, icon: Node<Msg>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text<'m: 'l>(mut self, text: &'m str) -> Self {
        self.text = Some(std::borrow::Cow::Borrowed(text));
        self
    }

    pub fn text_owned(mut self, text: String) -> Self {
        self.text = Some(std::borrow::Cow::Owned(text));
        self
    }

    pub fn name(mut self, name: &'l str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn match_text(&self, text: &str) -> bool {
        self.text
            .as_ref()
            .map(|t| t.to_lowercase().contains(text.to_lowercase().as_str()))
            .unwrap_or(true)
    }

    pub fn add_class<'m: 'l>(mut self, name: &'m str) -> Self {
        self.class_list.push(Cow::Borrowed(name));
        self
    }

    pub fn build(self, display_type: DisplayType) -> StyledSelectChild<'l> {
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
        class_list,
        variant,
    } = values;

    let label_class = match display_type {
        DisplayType::SelectOption => vec![
            "optionLabel",
            variant.to_str(),
            name.as_deref().unwrap_or_default(),
        ],
        DisplayType::SelectValue => vec![
            "selectItemLabel",
            variant.to_str(),
            name.as_deref().unwrap_or_default(),
        ],
    }
    .join(" ");

    let wrapper_class = match display_type {
        DisplayType::SelectOption => vec!["optionItem", name.as_deref().unwrap_or_default()],
        DisplayType::SelectValue => vec!["selectItem", name.as_deref().unwrap_or_default()],
    }
    .join(" ");

    let icon_node = match icon {
        Some(icon) => icon,
        _ => empty![],
    };

    let label_node = match text {
        Some(text) => div![
            attrs![
                At::Class => name.as_deref().map(|s| format!("{}Label", s)).unwrap_or_default(),
                At::Class => class_list.join(" "),
            ],
            class![label_class.as_str()],
            text
        ],
        _ => empty![],
    };

    div![
        class![variant.to_str()],
        class![wrapper_class.as_str()],
        attrs![At::Class => class_list.join(" ")],
        icon_node,
        label_node
    ]
}

impl<'l> ToChild<'l> for jirs_data::User {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let avatar = crate::shared::styled_avatar::StyledAvatar::build()
            .avatar_url(self.avatar_url.as_deref().unwrap_or_default())
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

impl<'l> ToChild<'l> for jirs_data::IssuePriority {
    type Builder = StyledSelectChildBuilder<'l>;
    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let icon = crate::shared::styled_icon::StyledIcon::build(self.clone().into())
            .add_class(self.to_str())
            .build()
            .into_node();
        let text = self.to_str();

        StyledSelectChild::build()
            .icon(icon)
            .value(self.clone().into())
            .text(text)
            .add_class(self.to_str())
    }
}

impl<'l> ToChild<'l> for jirs_data::IssueStatus {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChild::build()
            .value(self.id as u32)
            .add_class(self.name.as_str())
            .text(self.name.as_str())
    }
}

impl<'l> ToChild<'l> for jirs_data::IssueType {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let name = self.to_label();

        let type_icon = crate::shared::styled_icon::StyledIcon::build(self.clone().into())
            .add_class(name)
            .build()
            .into_node();

        StyledSelectChild::build()
            .add_class(name)
            .text(name)
            .icon(type_icon)
            .value(self.clone().into())
    }
}

impl<'l> ToChild<'l> for jirs_data::ProjectCategory {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChild::build()
            .add_class(self.to_str())
            .text(self.to_str())
            .value(self.clone().into())
    }
}

impl<'l> ToChild<'l> for jirs_data::UserRole {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let name = self.to_str();

        StyledSelectChild::build()
            .add_class(name)
            .add_class("capitalize")
            .text(name)
            .value(self.clone().into())
    }
}

impl<'l> ToChild<'l> for jirs_data::Project {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChild::build()
            .text(self.name.as_str())
            .value(self.id as u32)
    }
}

impl<'l> ToChild<'l> for jirs_data::Epic {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChild::build()
            .text(self.name.as_str())
            .value(self.id as u32)
    }
}

impl<'l> ToChild<'l> for u32 {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let name = stringify!(self);

        StyledSelectChild::build()
            .add_class(name)
            .text(name)
            .value(*self)
    }
}

pub type Label = String;
pub type Value = u32;

impl<'l> ToChild<'l> for (Label, Value) {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChild::build()
            .text(self.0.as_str())
            .value(self.1)
    }
}
