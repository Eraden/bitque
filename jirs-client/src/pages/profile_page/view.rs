use std::collections::HashMap;

use jirs_data::*;
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_image_input::StyledImageInput;
use crate::components::styled_input::{InputVariant, StyledInput};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectChild;
use crate::model::{Model, PageContent};
use crate::pages::profile_page::model::ProfilePage;
use crate::shared::{inner_layout, ToNode};
use crate::{FieldId, Msg, PageChanged, ProfilePageChange};

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return empty![],
    };

    let avatar = StyledImageInput {
        id: FieldId::Profile(UsersFieldId::Avatar),
        class_list: "avatar",
        url: page.avatar.url.as_deref(),
    }
    .into_node();

    let username = StyledInput {
        id: Some(FieldId::Profile(UsersFieldId::Username)),
        valid: page.name.is_valid(),
        value: page.name.value.as_str(),
        variant: InputVariant::Primary,
        ..Default::default()
    }
    .into_node();
    let username_field = StyledField {
        label: "Username",
        input: username,
        ..Default::default()
    }
    .into_node();

    let email = StyledInput {
        id: Some(FieldId::Profile(UsersFieldId::Email)),
        valid: page.email.is_valid(),
        value: page.email.value.as_str(),
        variant: InputVariant::Primary,
        ..Default::default()
    }
    .into_node();
    let email_field = StyledField {
        label: "E-Mail",
        input: email,
        ..Default::default()
    }
    .into_node();

    let current_project = build_current_project(model, page);

    let submit = StyledButton {
        variant: ButtonVariant::Primary,
        text: Some("Save"),
        on_click: Some(mouse_ev(Ev::Click, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Profile(ProfilePageChange::SubmitForm))
        })),
        ..Default::default()
    }
    .into_node();
    let submit_field = StyledField {
        input: submit,
        ..Default::default()
    }
    .into_node();

    let content = StyledForm::build()
        .heading("Profile")
        .on_submit(ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Profile(ProfilePageChange::SubmitForm))
        }))
        .add_field(avatar)
        .add_field(username_field)
        .add_field(email_field)
        .add_field(current_project)
        .add_field(submit_field)
        .build()
        .into_node();
    inner_layout(model, "profile", &[content])
}

fn build_current_project(model: &Model, page: &ProfilePage) -> Node<Msg> {
    let inner = if model.projects.len() <= 1 {
        let name = model
            .project
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or_default();
        span![name]
    } else {
        let mut project_by_id = HashMap::new();
        for p in model.projects.iter() {
            project_by_id.insert(p.id, p);
        }
        let mut joined_projects = HashMap::new();
        for p in model.user_projects.iter() {
            joined_projects.insert(p.project_id, p);
        }

        StyledSelect {
            id: FieldId::Profile(UsersFieldId::CurrentProject),
            name: "current_project",
            valid: true,
            opened: page.current_project.opened,
            text_filter: page.current_project.text_filter.as_str(),
            variant: SelectVariant::Normal,
            options: Some(model.projects.iter().filter_map(|project| {
                joined_projects
                    .get(&project.id)
                    .map(|_| project_select_option(project))
            })),
            selected: page
                .current_project
                .values
                .iter()
                .filter_map(|id| {
                    project_by_id
                        .remove(&((*id) as i32))
                        .map(project_select_option)
                })
                .collect(),
            ..Default::default()
        }
        .into_node()
    };
    StyledField {
        label: "Current project",
        input: div![C!["project-name"], inner],
        ..Default::default()
    }
    .into_node()
}

fn project_select_option<'l>(project: &'l Project) -> StyledSelectChild<'l> {
    StyledSelectChild {
        text: Some(project.name.as_str()),
        value: project.id as u32,
        ..Default::default()
    }
}
