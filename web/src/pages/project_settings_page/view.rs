use std::collections::HashMap;

use jirs_data::{IssueStatus, ProjectCategory, TimeTracking};
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_checkbox::{ChildBuilder, StyledCheckbox, StyledCheckboxState};
use crate::components::styled_editor::render_styled_editor;
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::{InputVariant, StyledInput};
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::components::styled_textarea::StyledTextarea;
use crate::model::{self, Model, PageContent};
use crate::pages::project_settings_page::{events, ProjectSettingsPage};
use crate::shared::inner_layout;
use crate::{FieldId, Msg, ProjectFieldId};

static TIME_TRACKING_FIBONACCI: &str = include_str!("./time_tracking_fibonacci.txt");
static TIME_TRACKING_HOURLY: &str = include_str!("./time_tracking_hourly.txt");

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return empty![],
    };
    let name_field = name_field(page);

    let url_field = url_field(page);

    let description_field = render_styled_editor(&page.description);

    let category_field = category_field(page);

    let time_tracking_field = time_tracking_select(page);

    let columns_field = columns_section(model, page);

    let save_button = StyledButton {
        class_list: "actionButton",
        on_click: Some(events::on_click_submit_save_changes()),
        text: Some("Save changes"),
        ..Default::default()
    }
    .render();

    let form = StyledForm {
        heading: "Project Details",
        fields: vec![
            name_field,
            url_field,
            description_field,
            category_field,
            time_tracking_field,
            save_button,
            columns_field,
        ],
        on_submit: Some(events::on_submit_submit_save_changes()),
    }
    .render();

    let project_section = [div![C!["formContainer"], form]];

    inner_layout(model, "projectSettings", &project_section)
}

#[inline(always)]
fn time_tracking_select(page: &ProjectSettingsPage) -> Node<Msg> {
    let time_tracking = StyledCheckbox {
        options: Some(
            TimeTracking::default()
                .into_iter()
                .map(|tt| time_tracking_checkbox_option(tt, &page.time_tracking)),
        ),
        class_list: "timeTracking",
    }
    .render();
    let time_tracking_type: TimeTracking = page.time_tracking.value.into();
    StyledField {
        input: time_tracking,
        tip: Some(match time_tracking_type {
            TimeTracking::Fibonacci => TIME_TRACKING_FIBONACCI,
            TimeTracking::Hourly => TIME_TRACKING_HOURLY,
            _ => "",
        }),
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn time_tracking_checkbox_option<'l>(
    t: TimeTracking,
    state: &StyledCheckboxState,
) -> ChildBuilder<'l> {
    let value: u32 = t.into();
    ChildBuilder {
        field_id: state.field_id.clone(),
        selected: state.value == value,
        label: match t {
            TimeTracking::Untracked => "No tracking",
            TimeTracking::Fibonacci => "Fibonacci (Bad mode)",
            TimeTracking::Hourly => "Evil Mode (Hourly)",
        },
        name: match t {
            TimeTracking::Untracked => "untracked",
            TimeTracking::Fibonacci => "fibonacci",
            TimeTracking::Hourly => "hourly",
        },
        class_list: match t {
            TimeTracking::Untracked => "untracked",
            TimeTracking::Fibonacci => "fibonacci",
            TimeTracking::Hourly => "hourly",
        },
        value,
        ..Default::default()
    }
}

/// Build project name input with styled field wrapper
#[inline(always)]
fn name_field(page: &ProjectSettingsPage) -> Node<Msg> {
    let name = StyledTextarea {
        id: Some(FieldId::ProjectSettings(ProjectFieldId::Name)),
        value: page.payload.name.as_deref().unwrap_or_default(),
        height: 39,
        max_height: 39,
        disable_auto_resize: true,
        ..Default::default()
    }
    .render();
    StyledField {
        label: "Name",
        input: name,
        tip: Some(""),
        ..Default::default()
    }
    .render()
}

/// Build project url input with styled field wrapper
#[inline(always)]
fn url_field(page: &ProjectSettingsPage) -> Node<Msg> {
    let url = StyledTextarea {
        id: Some(FieldId::ProjectSettings(ProjectFieldId::Url)),
        height: 39,
        max_height: 39,
        disable_auto_resize: true,
        value: page.payload.url.as_deref().unwrap_or_default(),
        ..Default::default()
    }
    .render();
    StyledField {
        label: "Url",
        input: url,
        tip: Some(""),
        ..Default::default()
    }
    .render()
}

/// Build project category dropdown with styled field wrapper
#[inline(always)]
fn category_field(page: &ProjectSettingsPage) -> Node<Msg> {
    let category = StyledSelect {
        id: FieldId::ProjectSettings(ProjectFieldId::Category),
        opened: page.project_category_state.opened,
        text_filter: page.project_category_state.text_filter.as_str(),
        valid: true,
        variant: SelectVariant::Normal,
        options: Some(
            ProjectCategory::default()
                .into_iter()
                .map(category_select_option),
        ),
        selected: vec![category_select_option(
            page.payload.category.as_ref().cloned().unwrap_or_default(),
        )],
        ..Default::default()
    }
    .render();
    StyledField {
        input: category,
        label: "Project Category",
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn category_select_option<'l>(pc: ProjectCategory) -> StyledSelectOption<'l> {
    StyledSelectOption {
        class_list: pc.to_str(),
        text: Some(pc.to_str()),
        value: pc.into(),
        ..Default::default()
    }
}

/// Build draggable columns preview with option to remove and add new columns
#[inline(always)]
fn columns_section(model: &Model, page: &ProjectSettingsPage) -> Node<Msg> {
    let width = 100f64 / (model.issue_statuses.len() + 1) as f64;
    let column_style = format!("width: calc({width}% - 10px)", width = width);
    let per_column_issue_count = model
        .issue_ids
        .iter()
        .filter_map(|id| model.issues_by_id.get(id))
        .iter()
        .fold(
            HashMap::with_capacity(model.issue_statuses_by_id.len()),
            |mut h, issue| {
                *h.entry(issue.issue_status_id).or_insert(0) += 1;
                h
            },
        );
    let columns: Vec<Node<Msg>> = model
        .issue_statuses
        .iter()
        .map(|is| column_preview(is, page, &per_column_issue_count, column_style.as_str()))
        .collect();

    let columns_section = section![
        C!["columnsSection"],
        div![
            C!["columns"],
            columns,
            add_column(page, column_style.as_str())
        ]
    ];
    StyledField {
        label: "Columns",
        tip: Some("Double-click on name to change it."),
        input: columns_section,
        class_list: "columnsField",
    }
    .render()
}

#[inline(always)]
fn add_column(page: &ProjectSettingsPage, column_style: &str) -> Node<Msg> {
    let on_click = events::on_click_add_status();

    if page.edit_column_id == Some(0) {
        let blur = events::on_focus_out_close_edit_status();
        let on_submit = events::on_submit_create_column();

        let input = StyledInput {
            value: page.name.value.as_str(),
            valid: page.name.is_valid(),
            auto_focus: true,
            variant: InputVariant::Primary,
            id: Some(FieldId::ProjectSettings(ProjectFieldId::IssueStatusName)),
            ..Default::default()
        }
        .render();

        div![
            C!["columnPreview"],
            div![C!["columnName"], form![on_submit, input]],
            blur
        ]
    } else {
        let add_column = StyledIcon::from(Icon::Plus).render();
        div![
            C!["columnPreview"],
            attrs![At::Style => column_style],
            div![C!["columnName addColumn"], add_column],
            on_click,
        ]
    }
}

#[inline(always)]
fn column_preview(
    is: &IssueStatus,
    page: &ProjectSettingsPage,
    per_column_issue_count: &HashMap<i32, i32>,
    column_style: &str,
) -> Node<Msg> {
    if page.edit_column_id == Some(is.id) {
        let blur = events::on_focus_out_close_edit_status();

        let input = StyledInput {
            value: page.name.value.as_str(),
            valid: page.name.is_valid(),
            variant: InputVariant::Primary,
            auto_focus: true,
            id: Some(FieldId::ProjectSettings(ProjectFieldId::IssueStatusName)),
            ..Default::default()
        }
        .render();

        div![C!["columnPreview"], div![C!["columnName"], input], blur]
    } else {
        show_column_preview(is, per_column_issue_count, column_style)
    }
}

#[inline(always)]
fn show_column_preview(
    is: &IssueStatus,
    per_column_issue_count: &HashMap<i32, i32>,
    column_style: &str,
) -> Node<Msg> {
    let drag_started = events::on_drag_start_start_drag_column(is.id);
    let drag_stopped = events::on_drag_end_stop_drag_column(is.id);
    let drag_over_handler = events::on_drag_over_exchange_position(is.id);
    let drag_out = events::on_drag_leave_leave_drag_column(is.id);
    let on_edit = events::on_click_edit_column(is.id);

    let issue_count_in_column = per_column_issue_count
        .get(&is.id)
        .cloned()
        .unwrap_or_default();
    let delete_row = if issue_count_in_column == 0 {
        let delete = StyledButton {
            variant: ButtonVariant::Primary,
            class_list: "removeColumn",
            icon: Some(StyledIcon::from(Icon::Trash).render()),
            on_click: Some(events::on_click_delete_column(is.id)),
            ..Default::default()
        }
        .render();
        div![C!["removeColumn"], delete]
    } else {
        div![
            C!["issueCount"],
            format!("Issues in column: {}", issue_count_in_column)
        ]
    };

    div![
        C!["columnPreview"],
        attrs![At::Style => column_style, At::Draggable => "true", At::DropZone => "true"],
        div![
            C!["columnName"],
            span![is.name.as_str()],
            on_edit,
            delete_row
        ],
        drag_started,
        drag_stopped,
        drag_over_handler,
        drag_out,
    ]
}
