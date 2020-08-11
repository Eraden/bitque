use seed::prelude::Orders;

use jirs_data::{Issue, IssueFieldId, WsMsg};

use crate::model::{ModalType, Model, Page, PageContent, ProjectPage};
use crate::shared::styled_select::StyledSelectChange;
use crate::ws::{board_load, send_ws_msg};
use crate::{BoardPageChange, EditIssueModalSection, FieldId, Msg, PageChanged, WebSocketChanged};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    match msg {
        Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..)) => {
            build_page_content(model);
        }
        _ => (),
    }

    let project_page = match &mut model.page_content {
        PageContent::Project(project_page) => project_page,
        _ => return,
    };

    match msg {
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(..)))
        | Msg::ProjectChanged(Some(..))
        | Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..)) => {
            board_load(model, orders);
        }
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueUpdated(issue))) => {
            let mut old: Vec<Issue> = vec![];
            std::mem::swap(&mut old, &mut model.issues);
            for is in old {
                if is.id == issue.id {
                    model.issues.push(issue.clone())
                } else {
                    model.issues.push(is);
                }
            }
        }
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueDeleted(id))) => {
            let mut old: Vec<Issue> = vec![];
            std::mem::swap(&mut old, &mut model.issues);
            for is in old {
                if is.id != id {
                    model.issues.push(is);
                }
            }
            orders.skip().send_msg(Msg::ModalDropped);
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
            StyledSelectChange::Text(text),
        ) => {
            let modal = model
                .modals
                .iter_mut()
                .filter_map(|modal| match modal {
                    ModalType::EditIssue(_, modal) => Some(modal),
                    _ => None,
                })
                .last();
            if let Some(m) = modal {
                m.top_type_state.text_filter = text;
            }
        }
        Msg::StrInputChanged(FieldId::TextFilterBoard, text) => {
            project_page.text_filter = text;
        }
        Msg::ProjectAvatarFilterChanged(user_id, active) => {
            if active {
                project_page.active_avatar_filters = project_page
                    .active_avatar_filters
                    .iter()
                    .filter_map(|id| if *id != user_id { Some(*id) } else { None })
                    .collect();
            } else {
                project_page.active_avatar_filters.push(user_id);
            }
        }
        Msg::ProjectToggleOnlyMy => {
            project_page.only_my_filter = !project_page.only_my_filter;
        }
        Msg::ProjectToggleRecentlyUpdated => {
            project_page.recently_updated_filter = !project_page.recently_updated_filter;
        }
        Msg::ProjectClearFilters => {
            project_page.active_avatar_filters = vec![];
            project_page.recently_updated_filter = false;
            project_page.only_my_filter = false;
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStarted(issue_id))) => {
            crate::ws::issue::drag_started(issue_id, model)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStopped(_))) => {
            crate::ws::issue::sync(model, orders);
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::ExchangePosition(
            issue_bellow_id,
        ))) => crate::ws::issue::exchange_position(issue_bellow_id, model),
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragOverStatus(status))) => {
            crate::ws::issue::change_status(status, model)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDropZone(_status))) => {
            crate::ws::issue::sync(model, orders)
        }
        Msg::PageChanged(PageChanged::Board(BoardPageChange::DragLeave(_id))) => {
            project_page.issue_drag.clear_last();
        }
        Msg::DeleteIssue(issue_id) => {
            send_ws_msg(
                jirs_data::WsMsg::IssueDelete(issue_id),
                model.ws.as_ref(),
                orders,
            );
        }
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::Project(Box::new(ProjectPage::default()));
}
