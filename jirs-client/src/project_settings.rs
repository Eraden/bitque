use std::collections::HashSet;

use seed::{prelude::*, *};
use wasm_bindgen::__rt::std::collections::HashMap;

use jirs_data::{
    IssueStatus, IssueStatusId, ProjectCategory, TimeTracking, ToVec, UpdateProjectPayload, WsMsg,
};

use crate::model::{
    DeleteIssueStatusModal, ModalType, Model, Page, PageContent, ProjectSettingsPage,
};
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_checkbox::StyledCheckbox;
use crate::shared::styled_editor::StyledEditor;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::{StyledSelect, StyledSelectChange};
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::{inner_layout, ToChild, ToNode};
use crate::ws::{enqueue_ws_msg, send_ws_msg};
use crate::FieldChange::TabChanged;
use crate::{
    model, FieldId, Msg, PageChanged, ProjectFieldId, ProjectPageChange, WebSocketChanged,
};

//######################################
// VIEW
//######################################

static TIME_TRACKING_FIBONACCI: &'static str = "Tracking employees’ time carries the risk of having them feel like they are being spied on. This is one of the most common fears that employees have when a time tracking system is implemented. No one likes to feel like they’re always being watched.";
static TIME_TRACKING_HOURLY: &'static str = "Employees may feel intimidated by demands to track their time. Or they could feel that they’re constantly being watched and evaluated. And for overly ambitious managers, employee time tracking may open the doors to excessive micromanaging.";

pub fn view(model: &model::Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return empty![],
    };
    let name_field = name_field(page);

    let url_field = url_field(page);

    let description_field = description_field(page);

    let category_field = category_field(page);

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
        .tip(match time_tracking_type {
            TimeTracking::Fibonacci => TIME_TRACKING_FIBONACCI,
            TimeTracking::Hourly => TIME_TRACKING_HOURLY,
            _ => "",
        })
        .build()
        .into_node();

    let columns_field = columns_section(model, page);

    let save_button = StyledButton::build()
        .add_class("actionButton")
        .on_click(mouse_ev(Ev::Click, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::ProjectSettings(
                ProjectPageChange::SubmitProjectSettingsForm,
            ))
        }))
        .text("Save changes")
        .build()
        .into_node();

    let form = StyledForm::build()
        .heading("Project Details")
        .on_submit(ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::ProjectSettings(
                ProjectPageChange::SubmitProjectSettingsForm,
            ))
        }))
        .add_field(name_field)
        .add_field(url_field)
        .add_field(description_field)
        .add_field(category_field)
        .add_field(time_tracking_field)
        .add_field(save_button)
        .add_field(columns_field)
        .build()
        .into_node();

    let project_section = vec![div![class!["formContainer"], form]];

    inner_layout(
        model,
        "projectSettings",
        project_section,
        crate::modal::view(model),
    )
}

/// Build project name input with styled field wrapper
fn name_field(page: &Box<ProjectSettingsPage>) -> Node<Msg> {
    let name = StyledTextarea::build(FieldId::ProjectSettings(ProjectFieldId::Name))
        .value(page.payload.name.as_ref().cloned().unwrap_or_default())
        .height(39)
        .max_height(39)
        .disable_auto_resize()
        .build()
        .into_node();
    StyledField::build()
        .label("Name")
        .input(name)
        .tip("")
        .build()
        .into_node()
}

/// Build project url input with styled field wrapper
fn url_field(page: &Box<ProjectSettingsPage>) -> Node<Msg> {
    let url = StyledTextarea::build(FieldId::ProjectSettings(ProjectFieldId::Url))
        .height(39)
        .max_height(39)
        .disable_auto_resize()
        .value(page.payload.url.as_ref().cloned().unwrap_or_default())
        .build()
        .into_node();
    StyledField::build()
        .label("Url")
        .input(url)
        .tip("")
        .build()
        .into_node()
}

/// Build project description text area with styled field wrapper
fn description_field(page: &Box<ProjectSettingsPage>) -> Node<Msg> {
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
    StyledField::build()
        .input(description)
        .label("Description")
        .tip("Describe the project in as much detail as you'd like.")
        .build()
        .into_node()
}

/// Build project category dropdown with styled field wrapper
fn category_field(page: &Box<ProjectSettingsPage>) -> Node<Msg> {
    let category = StyledSelect::build(FieldId::ProjectSettings(ProjectFieldId::Category))
        .opened(page.project_category_state.opened)
        .text_filter(page.project_category_state.text_filter.as_str())
        .valid(true)
        .normal()
        .options(
            ProjectCategory::ordered()
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        )
        .selected(vec![page
            .payload
            .category
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .to_child()])
        .build()
        .into_node();
    StyledField::build()
        .label("Project Category")
        .input(category)
        .build()
        .into_node()
}

fn build_page_content(model: &mut Model) {
    let project = match &model.project {
        Some(project) => project,
        _ => return,
    };
    model.page_content = PageContent::ProjectSettings(Box::new(ProjectSettingsPage::new(project)));
}

/// Build draggable columns preview with option to remove and add new columns
fn columns_section(model: &Model, page: &Box<ProjectSettingsPage>) -> Node<Msg> {
    let width = 100f64 / (model.issue_statuses.len() + 1) as f64;
    let column_style = format!("width: calc({width}% - 10px)", width = width);
    let mut per_column_issue_count = HashMap::new();
    for issue in model.issues.iter() {
        *per_column_issue_count
            .entry(issue.issue_status_id)
            .or_insert(0) += 1;
    }
    let columns: Vec<Node<Msg>> = model
        .issue_statuses
        .iter()
        .map(|is| column_preview(is, page, &per_column_issue_count, column_style.as_str()))
        .collect();

    let columns_section = section![
        class!["columnsSection"],
        div![
            class!["columns"],
            columns,
            add_column(page, column_style.as_str())
        ]
    ];
    StyledField::build()
        .add_class("columnsField")
        .input(columns_section)
        .label("Columns")
        .tip("Double-click on name to change it.")
        .build()
        .into_node()
}

fn add_column(page: &ProjectSettingsPage, column_style: &str) -> Node<Msg> {
    let on_click = mouse_ev(Ev::Click, move |_| {
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::EditIssueStatusName(Some(0)),
        ))
    });

    if page.edit_column_id == Some(0) {
        let blur = ev("focusout", |_| {
            Msg::PageChanged(PageChanged::ProjectSettings(
                ProjectPageChange::EditIssueStatusName(None),
            ))
        });
        let on_submit = ev(Ev::Submit, move |ev| {
            ev.prevent_default();
            Some(Msg::PageChanged(PageChanged::ProjectSettings(
                ProjectPageChange::SubmitIssueStatusForm,
            )))
        });

        let input = StyledInput::build(FieldId::ProjectSettings(ProjectFieldId::IssueStatusName))
            .state(&page.name)
            .primary()
            .auto_focus()
            .on_input_ev(blur)
            .build()
            .into_node();

        div![
            class!["columnPreview"],
            div![class!["columnName"], form![on_submit, input]]
        ]
    } else {
        let add_column = StyledIcon::build(Icon::Plus).build().into_node();
        div![
            class!["columnPreview"],
            attrs![At::Style => column_style],
            div![class!["columnName addColumn"], add_column],
            on_click,
        ]
    }
}

fn column_preview(
    is: &IssueStatus,
    page: &ProjectSettingsPage,
    per_column_issue_count: &HashMap<i32, i32>,
    column_style: &str,
) -> Node<Msg> {
    if page.edit_column_id == Some(is.id) {
        let blur = ev("focusout", |_| {
            Msg::PageChanged(PageChanged::ProjectSettings(
                ProjectPageChange::EditIssueStatusName(None),
            ))
        });
        let input = StyledInput::build(FieldId::ProjectSettings(ProjectFieldId::IssueStatusName))
            .state(&page.name)
            .primary()
            .auto_focus()
            .on_input_ev(blur)
            .build()
            .into_node();

        div![class!["columnPreview"], div![class!["columnName"], input]]
    } else {
        show_column_preview(is, per_column_issue_count, column_style)
    }
}

fn show_column_preview(
    is: &IssueStatus,
    per_column_issue_count: &HashMap<i32, i32>,
    column_style: &str,
) -> Node<Msg> {
    let id = is.id;
    let drag_started = drag_ev(Ev::DragStart, move |_| {
        Some(Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::ColumnDragStarted(id),
        )))
    });
    let drag_stopped = drag_ev(Ev::DragEnd, move |_| {
        Some(Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::ColumnDragStopped(id),
        )))
    });
    let drag_over_handler = drag_ev(Ev::DragOver, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Some(Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::ColumnExchangePosition(id),
        )))
    });
    let drag_out = drag_ev(Ev::DragLeave, move |_| {
        Some(Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::ColumnDragLeave(id),
        )))
    });

    let on_edit = mouse_ev(Ev::Click, move |_| {
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::EditIssueStatusName(Some(id)),
        ))
    });
    let issue_count_in_column = per_column_issue_count.get(&id).cloned().unwrap_or_default();
    let delete_row = if issue_count_in_column == 0 {
        let on_delete = mouse_ev(Ev::Click, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            Msg::ModalOpened(Box::new(ModalType::DeleteIssueStatusModal(Box::new(
                DeleteIssueStatusModal::new(id),
            ))))
        });
        let delete = StyledButton::build()
            .primary()
            .add_class("removeColumn")
            .icon(Icon::Trash)
            .on_click(on_delete)
            .build()
            .into_node();
        div![class!["removeColumn"], delete]
    } else {
        div![
            class!["issueCount"],
            format!("Issues in column: {}", issue_count_in_column)
        ]
    };

    div![
        class!["columnPreview"],
        attrs![At::Style => column_style, At::Draggable => "true", At::DropZone => "true"],
        div![
            class!["columnName"],
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

//#######################################
// Update
//#######################################

pub fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::ProjectSettings {
        return;
    }

    match msg {
        Msg::WebSocketChange(ref change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(..)) => {
                enqueue_ws_msg(
                    vec![
                        WsMsg::ProjectRequest,
                        WsMsg::IssueStatusesRequest,
                        WsMsg::ProjectIssuesRequest,
                    ],
                    model.ws.as_ref(),
                    orders,
                );
            }
            WebSocketChanged::WsMsg(WsMsg::ProjectLoaded(..)) => {
                build_page_content(model);
            }
            WebSocketChanged::WsMsg(WsMsg::IssueStatusCreated(_)) => {
                match &mut model.page_content {
                    PageContent::ProjectSettings(page) if Some(0) == page.edit_column_id => {
                        page.reset();
                    }
                    _ => (),
                };
            }
            _ => (),
        },
        Msg::ChangePage(Page::ProjectSettings) => {
            build_page_content(model);
            if model.user.is_some() {
                enqueue_ws_msg(
                    vec![
                        WsMsg::ProjectRequest,
                        WsMsg::IssueStatusesRequest,
                        WsMsg::ProjectIssuesRequest,
                    ],
                    model.ws.as_ref(),
                    orders,
                );
            }
        }
        _ => (),
    }

    if model.user.is_none() || model.project.is_none() {
        return;
    }

    let page = match &mut model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return error!("bad content type"),
    };
    page.project_category_state.update(&msg, orders);
    page.time_tracking.update(&msg);
    page.name.update(&msg);

    match msg {
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
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::SubmitProjectSettingsForm,
        )) => {
            send_ws_msg(
                WsMsg::ProjectUpdateRequest(UpdateProjectPayload {
                    id: page.payload.id,
                    name: page.payload.name.clone(),
                    url: page.payload.url.clone(),
                    description: page.payload.description.clone(),
                    category: page.payload.category.clone(),
                    time_tracking: Some(page.time_tracking.value.into()),
                }),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::ColumnDragStarted(
            issue_status_id,
        ))) => {
            page.column_drag.drag(issue_status_id);
        }
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::ColumnDragStopped(
            _issue_status_id,
        ))) => {
            sync(model, orders);
        }
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::ColumnDragLeave(
            _issue_status_id,
        ))) => page.column_drag.clear_last(),
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::ColumnExchangePosition(issue_bellow_id),
        )) => exchange_position(issue_bellow_id, model),
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::ColumnDropZone(
            _issue_status_id,
        ))) => {
            sync(model, orders);
        }
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::EditIssueStatusName(
            id,
        ))) => {
            if page.edit_column_id.is_some() && id.is_none() {
                let old_id = page.edit_column_id.as_ref().cloned();
                let name = page.name.value.clone();
                if let Some((id, pos)) = model
                    .issue_statuses
                    .iter()
                    .find(|is| Some(is.id) == old_id)
                    .map(|is| (is.id, is.position))
                {
                    send_ws_msg(
                        WsMsg::IssueStatusUpdate(id, name.to_string(), pos),
                        model.ws.as_ref(),
                        orders,
                    );
                }
            }
            page.name.value = model
                .issue_statuses
                .iter()
                .find_map(|is| {
                    if Some(is.id) == id {
                        Some(is.name.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            page.edit_column_id = id;
        }
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::SubmitIssueStatusForm,
        )) => {
            let name = page.name.value.clone();
            let position = model.issue_statuses.len();
            let ws_msg = WsMsg::IssueStatusCreate(name, position as i32);
            send_ws_msg(ws_msg, model.ws.as_ref(), orders);
        }
        _ => (),
    }
}

fn exchange_position(bellow_id: IssueStatusId, model: &mut Model) {
    let page = match &mut model.page_content {
        PageContent::ProjectSettings(page) => page,
        _ => return,
    };
    if page.column_drag.dragged_or_last(bellow_id) {
        return;
    }
    let dragged_id = match page.column_drag.dragged_id.as_ref().cloned() {
        Some(id) => id,
        _ => return error!("Nothing is dragged"),
    };

    let mut below = None;
    let mut dragged = None;
    let mut issues_statuses = vec![];
    std::mem::swap(&mut issues_statuses, &mut model.issue_statuses);

    for issue_status in issues_statuses.into_iter() {
        match issue_status.id {
            id if id == bellow_id => below = Some(issue_status),
            id if id == dragged_id => dragged = Some(issue_status),
            _ => model.issue_statuses.push(issue_status),
        };
    }

    let mut below = match below {
        Some(below) => below,
        _ => return,
    };
    let mut dragged = match dragged {
        Some(issue_status) => issue_status,
        _ => {
            model.issue_statuses.push(below);
            return;
        }
    };
    std::mem::swap(&mut dragged.position, &mut below.position);

    page.column_drag.mark_dirty(dragged.id);
    page.column_drag.mark_dirty(below.id);

    model.issue_statuses.push(below);
    model.issue_statuses.push(dragged);
    model
        .issue_statuses
        .sort_by(|a, b| a.position.cmp(&b.position));
    page.column_drag.last_id = Some(bellow_id);
}

fn sync(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let dirty = match &mut model.page_content {
        PageContent::ProjectSettings(page) => {
            let mut old = HashSet::new();
            std::mem::swap(&mut old, &mut page.column_drag.dirty);
            old
        }
        _ => return error!("bad content type"),
    };
    for id in dirty {
        let IssueStatus { name, position, .. } =
            match model.issue_statuses.iter().find(|is| is.id == id) {
                Some(is) => is,
                _ => continue,
            };
        send_ws_msg(
            WsMsg::IssueStatusUpdate(id, name.clone(), *position),
            model.ws.as_ref(),
            orders,
        );
    }
}
