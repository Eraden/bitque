use std::collections::HashSet;

use seed::error;
use seed::prelude::Orders;

use jirs_data::{IssueStatus, IssueStatusId, ProjectFieldId, UpdateProjectPayload, WsMsg};

use crate::model::{Model, Page, PageContent, ProjectSettingsPage};
use crate::shared::styled_select::StyledSelectChanged;
use crate::ws::{board_load, send_ws_msg};
use crate::FieldChange::TabChanged;
use crate::{FieldId, Msg, PageChanged, ProjectPageChange, WebSocketChanged};

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.page != Page::ProjectSettings {
        return;
    }

    match msg {
        Msg::ProjectChanged(Some(_)) => {
            build_page_content(model);
        }
        Msg::WebSocketChange(ref change) => match change {
            WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(..)) => {
                board_load(model, orders);
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
                board_load(model, orders);
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
    // page.description_rte.update(&msg, orders);

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
                WsMsg::ProjectUpdateLoad(UpdateProjectPayload {
                    id: page.payload.id,
                    name: page.payload.name.clone(),
                    url: page.payload.url.clone(),
                    description: page.payload.description.clone(),
                    category: page.payload.category,
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
                        WsMsg::IssueStatusUpdate(id, name, pos),
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

fn build_page_content(model: &mut Model) {
    let project = match &model.project {
        Some(project) => project,
        _ => return,
    };
    model.page_content = PageContent::ProjectSettings(Box::new(ProjectSettingsPage::new(project)));
}
