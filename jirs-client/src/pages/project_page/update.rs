use {
    crate::{
        model::{ModalType, Model, Page, PageContent},
        pages::project_page::model::ProjectPage,
        shared::styled_select::StyledSelectChanged,
        ws::{board_load, send_ws_msg},
        BoardPageChange, EditIssueModalSection, FieldId, Msg, OperationKind, PageChanged,
        ResourceKind,
    },
    jirs_data::*,
    seed::prelude::Orders,
};

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
        Msg::UserChanged(..)
        | Msg::ProjectChanged(Some(..))
        | Msg::ChangePage(Page::Project)
        | Msg::ChangePage(Page::AddIssue)
        | Msg::ChangePage(Page::EditIssue(..)) => {
            board_load(model, orders);
        }
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::SingleRemoved, ..) => {
            orders.skip().send_msg(Msg::ModalDropped);
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::ResourceChanged(
            ResourceKind::Issue
            | ResourceKind::Project
            | ResourceKind::IssueStatus
            | ResourceKind::Epic,
            ..,
        ) => {
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
            StyledSelectChanged::Text(text),
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
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::ProjectAvatarFilterChanged(user_id, active) => {
            if active {
                project_page.active_avatar_filters =
                    std::mem::replace(&mut project_page.active_avatar_filters, vec![])
                        .into_iter()
                        .filter(|id| *id != user_id)
                        .collect();
            } else {
                project_page.active_avatar_filters.push(user_id);
            }
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::ProjectToggleOnlyMy => {
            project_page.only_my_filter = !project_page.only_my_filter;
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::ProjectToggleRecentlyUpdated => {
            project_page.recently_updated_filter = !project_page.recently_updated_filter;
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
        }
        Msg::ProjectClearFilters => {
            project_page.active_avatar_filters = vec![];
            project_page.recently_updated_filter = false;
            project_page.only_my_filter = false;
            project_page.rebuild_visible(
                &model.epics,
                &model.issue_statuses,
                &model.issues,
                &model.user,
            );
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
