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
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::{Model, PageContent};
use crate::pages::profile_page::model::ProfilePage;
use crate::shared::inner_layout;
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
    .render();

    let username = StyledInput {
        id: Some(FieldId::Profile(UsersFieldId::Username)),
        valid: page.name.is_valid(),
        value: page.name.value.as_str(),
        variant: InputVariant::Primary,
        ..Default::default()
    }
    .render();
    let username_field = StyledField {
        label: "Username",
        input: username,
        ..Default::default()
    }
    .render();

    let email = StyledInput {
        id: Some(FieldId::Profile(UsersFieldId::Email)),
        valid: page.email.is_valid(),
        value: page.email.value.as_str(),
        variant: InputVariant::Primary,
        ..Default::default()
    }
    .render();
    let email_field = StyledField {
        label: "E-Mail",
        input: email,
        ..Default::default()
    }
    .render();

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
    .render();
    let submit_field = StyledField {
        input: submit,
        ..Default::default()
    }
    .render();

    let content = StyledForm {
        heading: "Profile",
        on_submit: Some(ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Profile(ProfilePageChange::SubmitForm))
        })),
        fields: vec![
            avatar,
            username_field,
            email_field,
            current_project,
            submit_field,
        ],
    }
    .render();
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
        .render()
    };
    StyledField {
        label: "Current project",
        input: div![C!["project-name"], inner],
        ..Default::default()
    }
    .render()
}

fn project_select_option<'l>(project: &'l Project) -> StyledSelectOption<'l> {
    StyledSelectOption {
        text: Some(project.name.as_str()),
        value: project.id as u32,
        ..Default::default()
    }
}
