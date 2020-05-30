use std::collections::HashMap;

use seed::{prelude::*, *};

use jirs_data::*;

use crate::model::{Model, PageContent, ProfilePage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_image_input::StyledImageInput;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::StyledSelect;
use crate::shared::{inner_layout, ToChild, ToNode};
use crate::{FieldId, Msg, PageChanged, ProfilePageChange};

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Profile(profile_page) => profile_page,
        _ => return empty![],
    };

    let avatar = StyledImageInput::build(FieldId::Profile(UsersFieldId::Avatar))
        .add_class("avatar")
        .state(&page.avatar)
        .build()
        .into_node();

    let username = StyledInput::build(FieldId::Profile(UsersFieldId::Username))
        .state(&page.name)
        .valid(true)
        .primary()
        .build()
        .into_node();
    let username_field = StyledField::build()
        .label("Username")
        .input(username)
        .build()
        .into_node();

    let email = StyledInput::build(FieldId::Profile(UsersFieldId::Username))
        .state(&page.email)
        .valid(true)
        .primary()
        .build()
        .into_node();
    let email_field = StyledField::build()
        .label("E-Mail")
        .input(email)
        .build()
        .into_node();

    let current_project = build_current_project(model, page);

    let submit = StyledButton::build()
        .primary()
        .text("Save")
        .on_click(mouse_ev(Ev::Click, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Profile(ProfilePageChange::SubmitForm))
        }))
        .build()
        .into_node();
    let submit_field = StyledField::build().input(submit).build().into_node();

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
    inner_layout(model, "profile", vec![content])
}

fn build_current_project(model: &Model, page: &Box<ProfilePage>) -> Node<Msg> {
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

        StyledSelect::build(FieldId::Profile(UsersFieldId::CurrentProject))
            .name("current_project")
            .normal()
            .options(
                model
                    .projects
                    .iter()
                    .filter_map(|project| {
                        joined_projects.get(&project.id).map(|_| project.to_child())
                    })
                    .collect(),
            )
            .selected(
                page.current_project
                    .values
                    .iter()
                    .filter_map(|id| project_by_id.get(&((*id) as i32)).map(|p| p.to_child()))
                    .collect(),
            )
            .state(&page.current_project)
            .build()
            .into_node()
    };
    StyledField::build()
        .label("Current project")
        .input(div![class!["project-name"], inner])
        .build()
        .into_node()
}
