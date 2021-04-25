use std::collections::HashMap;

use jirs_data::*;
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_checkbox::{ChildBuilder, StyledCheckbox, StyledCheckboxState};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_image_input::StyledImageInput;
use crate::components::styled_input::{InputVariant, StyledInput};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::Model;
use crate::pages::profile_page::model::ProfilePage;
use crate::shared::inner_layout;
use crate::{match_page, FieldId, Msg, PageChanged, ProfilePageChange};

pub fn view(model: &Model) -> Node<Msg> {
    let page = match_page!(model, Profile; Empty);

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
            editor_mode_select(page),
            current_project,
            submit_field,
        ],
    }
    .render();
    inner_layout(model, "profile", &[div![C!["formContainer"], content]])
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

#[inline(always)]
fn project_select_option<'l>(project: &'l Project) -> StyledSelectOption<'l> {
    StyledSelectOption {
        text: Some(project.name.as_str()),
        value: project.id as u32,
        ..Default::default()
    }
}

#[inline(always)]
fn editor_mode_select(page: &ProfilePage) -> Node<Msg> {
    let time_tracking = StyledCheckbox {
        options: Some(
            vec![
                TextEditorMode::MdOnly,
                TextEditorMode::RteOnly,
                TextEditorMode::Mixed,
            ]
            .into_iter()
            .map(|tem| editor_mode_checkbox_option(tem, &page.text_editor_mode)),
        ),
        class_list: "timeTracking",
    }
    .render();
    StyledField {
        input: time_tracking,
        label: "Text editor type",
        tip: Some("You can choose if what kind of text editor you will have"),
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn editor_mode_checkbox_option<'l>(
    tem: TextEditorMode,
    state: &StyledCheckboxState,
) -> ChildBuilder<'l> {
    let value: u32 = tem.into();
    ChildBuilder {
        field_id: state.field_id.clone(),
        selected: state.value == value,
        label: match tem {
            TextEditorMode::MdOnly => "Simple Markdown editor",
            TextEditorMode::RteOnly => "Advanced Rich Text Editor",
            TextEditorMode::Mixed => "Editor with possibility to switch between modes",
        },
        class_list: tem.to_str(),
        value,
        ..Default::default()
    }
}
