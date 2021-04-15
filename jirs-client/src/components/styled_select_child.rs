use {
    crate::{
        components::styled_select::SelectVariant,
        shared::{IntoChild, ToChild, ToNode},
        Msg,
    },
    seed::{prelude::*, *},
};

use crate::components::styled_avatar::StyledAvatar;
use crate::components::styled_icon::StyledIcon;

pub enum DisplayType {
    SelectOption,
    SelectValue,
}

pub struct StyledSelectChild<'l> {
    pub name: Option<&'l str>,
    pub icon: Option<Node<Msg>>,
    pub text: Option<&'l str>,
    pub display_type: DisplayType,
    pub value: u32,
    pub class_list: &'l str,
    pub variant: SelectVariant,
}

impl<'l> Default for StyledSelectChild<'l> {
    fn default() -> Self {
        Self {
            name: None,
            icon: None,
            text: None,
            display_type: DisplayType::SelectOption,
            value: 0,
            class_list: "",
            variant: Default::default(),
        }
    }
}

impl<'l> StyledSelectChild<'l> {
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
    pub icon: Option<Node<Msg>>,
    pub text: Option<&'l str>,
    pub name: Option<&'l str>,
    pub value: u32,
    pub class_list: &'l str,
    pub variant: SelectVariant,
}

impl<'l> Default for StyledSelectChildBuilder<'l> {
    fn default() -> Self {
        Self {
            icon: None,
            text: None,
            name: None,
            value: 0,
            class_list: "",
            variant: Default::default(),
        }
    }
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
        self.text = Some(text);
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

    pub fn class_list<'m: 'l>(mut self, name: &'m str) -> Self {
        self.class_list = name;
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

    let icon_node = match icon {
        Some(icon) => icon,
        _ => empty![],
    };

    let label_node = match text {
        Some(text) => div![
            C![
                variant.to_str(),
                name.as_deref()
                    .map(|s| format!("{}Label", s))
                    .unwrap_or_default(),
                match display_type {
                    DisplayType::SelectOption => "optionLabel",
                    DisplayType::SelectValue => "selectItemLabel",
                },
                class_list
            ],
            text
        ],
        _ => empty![],
    };

    div![
        C![
            variant.to_str(),
            name.as_deref().unwrap_or_default(),
            match display_type {
                DisplayType::SelectOption => "optionItem",
                DisplayType::SelectValue => "selectItem",
            },
            class_list
        ],
        icon_node,
        label_node
    ]
}

impl<'l> ToChild<'l> for jirs_data::User {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let avatar = StyledAvatar {
            size: 20,
            name: &self.name,
            avatar_url: self.avatar_url.as_deref(),
            ..StyledAvatar::default()
        }
        .into_node();
        StyledSelectChildBuilder {
            value: self.id as u32,
            icon: Some(avatar),
            text: Some(self.name.as_str()),
            ..Default::default()
        }
    }
}

impl<'l> IntoChild<'l> for jirs_data::IssuePriority {
    type Builder = StyledSelectChildBuilder<'l>;

    fn into_child(self) -> Self::Builder {
        let icon = StyledIcon {
            icon: self.clone().into(),
            class_list: self.to_str(),
            ..Default::default()
        }
        .into_node();

        StyledSelectChildBuilder {
            icon: Some(icon),
            text: Some(self.to_str()),
            class_list: self.to_str(),
            value: self.into(),
            ..Default::default()
        }
    }
}

impl<'l> ToChild<'l> for jirs_data::IssueStatus {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChildBuilder {
            value: self.id as u32,
            class_list: self.name.as_str(),
            text: Some(self.name.as_str()),
            ..Default::default()
        }
    }
}

impl<'l> IntoChild<'l> for jirs_data::IssueType {
    type Builder = StyledSelectChildBuilder<'l>;

    fn into_child(self) -> Self::Builder {
        let name = self.to_label();

        let type_icon = StyledIcon {
            icon: self.clone().into(),
            class_list: name,
            ..Default::default()
        }
        .into_node();

        StyledSelectChildBuilder {
            class_list: name,
            text: Some(name),
            icon: Some(type_icon),
            value: self.into(),
            ..Default::default()
        }
    }
}

impl<'l> IntoChild<'l> for jirs_data::ProjectCategory {
    type Builder = StyledSelectChildBuilder<'l>;

    fn into_child(self) -> Self::Builder {
        StyledSelectChildBuilder {
            class_list: self.to_str(),
            text: Some(self.to_str()),
            value: self.into(),
            ..Default::default()
        }
    }
}

impl<'l> IntoChild<'l> for jirs_data::UserRole {
    type Builder = StyledSelectChildBuilder<'l>;

    fn into_child(self) -> Self::Builder {
        let name = self.to_str();

        StyledSelectChildBuilder {
            text: Some(name),
            value: self.into(),
            class_list: name,
            ..Default::default()
        }
    }
}

macro_rules! id_name_builder {
    () => {
        fn to_child<'m: 'l>(&'m self) -> Self::Builder {
            StyledSelectChildBuilder {
                text: Some(self.name.as_str()),
                value: self.id as u32,
                ..Default::default()
            }
        }
    };
}

impl<'l> ToChild<'l> for jirs_data::Project {
    type Builder = StyledSelectChildBuilder<'l>;
    id_name_builder!();
}

impl<'l> ToChild<'l> for jirs_data::Epic {
    type Builder = StyledSelectChildBuilder<'l>;
    id_name_builder!();
}

impl<'l> ToChild<'l> for u32 {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        let name = stringify!(self);

        StyledSelectChildBuilder {
            class_list: name,
            text: Some(name),
            value: *self,
            ..Default::default()
        }
    }
}

pub type Label = String;
pub type Value = u32;

impl<'l> ToChild<'l> for (Label, Value) {
    type Builder = StyledSelectChildBuilder<'l>;

    fn to_child<'m: 'l>(&'m self) -> Self::Builder {
        StyledSelectChildBuilder {
            text: Some(self.0.as_str()),
            value: self.1,
            ..Default::default()
        }
    }
}
