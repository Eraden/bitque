use std::collections::HashSet;

use jirs_data::msg::{WsMsgIssueStatus, WsMsgProject, WsMsgSession};
use jirs_data::{IssueStatus, IssueStatusId, ProjectFieldId, UpdateProjectPayload, WsMsg};
use seed::prelude::Orders;

use crate::components::styled_select::StyledSelectChanged;
use crate::model::{Model, Page, PageContent};
use crate::pages::project_settings_page::ProjectSettingsPage;
use crate::ws::{board_load, send_ws_msg, sort_issue_statuses};
use crate::{match_page_mut, FieldId, Msg, PageChanged, ProjectPageChange, WebSocketChanged};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::ProjectSettings {
        return;
    }

    match msg {
        Msg::ProjectChanged(Some(_)) => {
            build_page_content(model, orders);
        }
        Msg::WebSocketChange(ref change) => match change {
            WebSocketChanged::WsMsg(WsMsg::Session(WsMsgSession::AuthorizeLoaded(..))) => {
                board_load(model, orders);
            }
            WebSocketChanged::WsMsg(WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusCreated(
                _,
            ))) => {
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
            build_page_content(model, orders);
            if model.user.is_some() {
                board_load(model, orders);
            }
        }
        _ => (),
    }

    if model.user.is_none() {
        return;
    }

    if model.project.is_none() {
        board_load(model, orders);
        return;
    }

    if matches!(model.page, Page::ProjectSettings)
        && !matches!(model.page_content, PageContent::ProjectSettings(..))
    {
        build_page_content(model, orders);
    }

    let page = match_page_mut!(model, ProjectSettings);

    page.project_category_state.update(&msg, orders);
    page.time_tracking.update(&msg);
    page.name.update(&msg);
    page.description.update(&msg, orders);

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
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            page.payload.category = Some(value.into());
        }
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::SubmitProjectSettingsForm,
        )) => {
            send_ws_msg(
                WsMsgProject::ProjectUpdateLoad(UpdateProjectPayload {
                    id: page.payload.id,
                    name: page.payload.name.clone(),
                    url: page.payload.url.clone(),
                    description: page.payload.description.clone(),
                    category: page.payload.category,
                    time_tracking: Some(page.time_tracking.value.into()),
                })
                .into(),
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
        )) => swap_position(issue_bellow_id, model),
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::ColumnDropZone(
            _issue_status_id,
        ))) => {
            // sync(model, orders);
        }
        Msg::PageChanged(PageChanged::ProjectSettings(ProjectPageChange::EditIssueStatusName(
            id,
        ))) => {
            if page.edit_column_id.is_some() && id.is_none() {
                if let Some(old_id) = page.edit_column_id {
                    let name = page.name.value.clone();
                    if let Some((id, pos)) = model
                        .issue_statuses_by_id
                        .get(&old_id)
                        .map(|is| (is.id, is.position))
                    {
                        send_ws_msg(
                            WsMsgIssueStatus::IssueStatusUpdate(id, name, pos).into(),
                            model.ws.as_ref(),
                            orders,
                        );
                    }
                }
            }
            if let Some(id) = id {
                page.name.value = model
                    .issue_statuses_by_id
                    .get(&id)
                    .map(|is| is.name.clone())
                    .unwrap_or_default();
            }
            page.edit_column_id = id;
        }
        Msg::PageChanged(PageChanged::ProjectSettings(
            ProjectPageChange::SubmitIssueStatusForm,
        )) => {
            let name = page.name.value.clone();
            let position = model.issue_status_ids.len();
            let ws_msg = WsMsgIssueStatus::IssueStatusCreate(name, position as i32).into();
            send_ws_msg(ws_msg, model.ws.as_ref(), orders);
        }
        _ => (),
    }
}

fn swap_position(bellow_id: IssueStatusId, model: &mut Model) {
    let page = crate::match_page_mut!(model, ProjectSettings);
    if page.column_drag.dragged_or_last(bellow_id) {
        return;
    }
    let dragged_id = match page.column_drag.dragged_id.as_ref().cloned() {
        Some(id) => id,
        _ => return log::error!("Nothing is dragged"),
    };

    let bellow = model
        .issue_statuses_by_id
        .get(&bellow_id)
        .map(|is| is.position)
        .unwrap_or_default();
    let dragged = model
        .issue_statuses_by_id
        .get(&dragged_id)
        .map(|is| is.position)
        .unwrap_or_default();

    if let Some(is) = model.issue_statuses_by_id.get_mut(&dragged_id) {
        is.position = bellow;
    }
    if let Some(is) = model.issue_statuses_by_id.get_mut(&bellow_id) {
        is.position = dragged;
    }

    page.column_drag.mark_dirty(dragged_id);
    page.column_drag.mark_dirty(bellow_id);

    page.column_drag.last_id = Some(bellow_id);
    sort_issue_statuses(model);
}

fn sync(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let dirty = match &mut model.page_content {
        PageContent::ProjectSettings(page) => {
            let mut old = HashSet::new();
            std::mem::swap(&mut old, &mut page.column_drag.dirty);
            old
        }
        _ => return log::error!("bad content type"),
    };
    for id in dirty {
        let IssueStatus { name, position, .. } = match model.issue_statuses_by_id.get(&id) {
            Some(is) => is,
            _ => continue,
        };
        send_ws_msg(
            WsMsgIssueStatus::IssueStatusUpdate(id, name.clone(), *position).into(),
            model.ws.as_ref(),
            orders,
        );
    }
}

fn build_page_content(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if matches!(model.page_content, PageContent::ProjectSettings(..)) {
        return;
    }
    if let Some(project) = &model.project {
        let mode = model
            .user_settings
            .as_ref()
            .map(|us| us.text_editor_mode)
            .unwrap_or_default();
        model.page_content =
            PageContent::ProjectSettings(Box::new(ProjectSettingsPage::new(mode, project)));
    }

    send_ws_msg(
        WsMsgIssueStatus::IssueStatusesLoad.into(),
        model.ws.as_ref(),
        orders,
    );
}
