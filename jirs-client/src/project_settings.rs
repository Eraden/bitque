use seed::{prelude::*, *};

use jirs_data::{ProjectCategory, TimeTracking, ToVec, WsMsg};

use crate::api::send_ws_msg;
use crate::model::{Model, Page, PageContent, ProjectSettingsPage};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_checkbox::StyledCheckbox;
use crate::shared::styled_editor::StyledEditor;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_select::{StyledSelect, StyledSelectChange};
use crate::shared::styled_select_child::ToStyledSelectChild;
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::{inner_layout, ToChild, ToNode};
use crate::FieldChange::TabChanged;
use crate::{model, FieldId, Msg, ProjectFieldId};

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    if model.page != Page::ProjectSettings {
        return;
    }

    match msg {
        Msg::WsMsg(WsMsg::AuthorizeLoaded(..)) => {
            send_ws_msg(WsMsg::ProjectRequest);
        }
        Msg::ChangePage(Page::ProjectSettings) => {
            send_ws_msg(WsMsg::ProjectRequest);
            build_page_content(model);
        }
        Msg::WsMsg(WsMsg::ProjectLoaded(..)) => {
            build_page_content(model);
        }
        _ => (),
    }

    let page = match &mut model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return,
    };
    page.project_category_state.update(&msg, orders);
    page.time_tracking.update(&msg);

    match msg {
        Msg::ProjectSaveChanges => send_ws_msg(WsMsg::ProjectUpdateRequest(page.payload.clone())),
        Msg::StrInputChanged(FieldId::ProjectSettings(ProjectFieldId::Name), text) => {
            page.payload.name = Some(text);
        }
        Msg::StrInputChanged(FieldId::ProjectSettings(ProjectFieldId::Url), text) => {
            page.payload.url = Some(text);
        }
        Msg::StrInputChanged(FieldId::ProjectSettings(ProjectFieldId::Description), text) => {
            page.payload.description = Some(text);
        }
        Msg::StyledSelectChanged(
            FieldId::ProjectSettings(ProjectFieldId::Category),
            StyledSelectChange::Changed(value),
        ) => {
            let category = value.into();
            page.payload.category = Some(category);
        }
        Msg::ModalChanged(TabChanged(
            FieldId::ProjectSettings(ProjectFieldId::Description),
            mode,
        )) => {
            page.description_mode = mode;
        }
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    let project = match &model.project {
        Some(project) => project,
        _ => return,
    };
    model.page_content = PageContent::ProjectSettings(Box::new(ProjectSettingsPage::new(project)));
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return empty![],
    };
    let name = StyledTextarea::build(FieldId::ProjectSettings(ProjectFieldId::Name))
        .value(page.payload.name.as_ref().cloned().unwrap_or_default())
        .height(39)
        .max_height(39)
        .disable_auto_resize()
        .build()
        .into_node();
    let name_field = StyledField::build()
        .label("Name")
        .input(name)
        .tip("")
        .build()
        .into_node();

    let url = StyledTextarea::build(FieldId::ProjectSettings(ProjectFieldId::Url))
        .height(39)
        .max_height(39)
        .disable_auto_resize()
        .value(page.payload.url.as_ref().cloned().unwrap_or_default())
        .build()
        .into_node();
    let url_field = StyledField::build()
        .label("Url")
        .input(url)
        .tip("")
        .build()
        .into_node();

    let description = StyledEditor::build(FieldId::ProjectSettings(ProjectFieldId::Description))
        .text(
            page.payload
                .description
                .as_ref()
                .cloned()
                .unwrap_or_default(),
        )
        .update_on(Ev::Change)
        .mode(page.description_mode.clone())
        .build()
        .into_node();
    let description_field = StyledField::build()
        .input(description)
        .label("Description")
        .tip("Describe the project in as much detail as you'd like.")
        .build()
        .into_node();

    let category = StyledSelect::build(FieldId::ProjectSettings(ProjectFieldId::Category))
        .opened(page.project_category_state.opened)
        .text_filter(page.project_category_state.text_filter.as_str())
        .valid(true)
        .normal()
        .options(
            ProjectCategory::ordered()
                .into_iter()
                .map(|c| c.to_select_child())
                .collect(),
        )
        .selected(vec![page
            .payload
            .category
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .to_select_child()])
        .build()
        .into_node();
    let category_field = StyledField::build()
        .label("Project Category")
        .input(category)
        .build()
        .into_node();

    let time_tracking =
        StyledCheckbox::build(FieldId::ProjectSettings(ProjectFieldId::TimeTracking))
            .options(vec![
                TimeTracking::Untracked.to_child(),
                TimeTracking::Fibonacci.to_child(),
                TimeTracking::Hourly.to_child(),
            ])
            .state(&page.time_tracking)
            .add_class("timeTracking")
            .build()
            .into_node();
    let time_tracking_type: TimeTracking = page.time_tracking.value.into();
    let time_tracking_field = StyledField::build()
        .input(time_tracking)
        .tip(if time_tracking_type == TimeTracking::Hourly {
            "Employees may feel intimidated by demands to track their time. Or they could feel that they’re constantly being watched and evaluated. And for overly ambitious managers, employee time tracking may open the doors to excessive micromanaging."
        } else {
            ""
        })
        .build()
        .into_node();

    let save_button = StyledButton::build()
        .add_class("actionButton")
        .on_click(mouse_ev(Ev::Click, |_| Msg::ProjectSaveChanges))
        .text("Save changes")
        .build()
        .into_node();

    let form = StyledForm::build()
        .heading("Project Details")
        .add_field(name_field)
        .add_field(url_field)
        .add_field(description_field)
        .add_field(category_field)
        .add_field(time_tracking_field)
        .add_field(save_button)
        .build()
        .into_node();

    let project_section = vec![div![class!["formContainer"], form]];

    inner_layout(model, "projectSettings", project_section, empty![])
}
