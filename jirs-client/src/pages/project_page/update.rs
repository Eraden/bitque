use {
    crate::{
        components::styled_select::StyledSelectChanged,
        model::{Model, Page, PageContent},
        pages::project_page::model::ProjectPage,
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

    let mut rebuild_visible = false;
    {
        let project_page = crate::match_page_mut!(model, Project);

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
                rebuild_visible = true;
            }
            Msg::ResourceChanged(
                ResourceKind::Issue
                | ResourceKind::Project
                | ResourceKind::IssueStatus
                | ResourceKind::Epic,
                ..,
            ) => {
                rebuild_visible = true;
            }
            Msg::StyledSelectChanged(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
                StyledSelectChanged::Text(text),
            ) => {
                if let Some(m) = &mut model.modals_mut().edit_issue {
                    m.top_type_state.text_filter = text;
                }
            }
            Msg::StrInputChanged(FieldId::TextFilterBoard, text) => {
                project_page.text_filter = text;
                rebuild_visible = true;
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
                rebuild_visible = true;
            }
            Msg::ProjectToggleOnlyMy => {
                project_page.only_my_filter = !project_page.only_my_filter;
                rebuild_visible = true;
            }
            Msg::ProjectToggleRecentlyUpdated => {
                project_page.recently_updated_filter = !project_page.recently_updated_filter;
                rebuild_visible = true;
            }
            Msg::ProjectClearFilters => {
                project_page.active_avatar_filters = vec![];
                project_page.recently_updated_filter = false;
                project_page.only_my_filter = false;
                rebuild_visible = true;
            }
            Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStarted(issue_id))) => {
                crate::ws::issue::drag_started(issue_id, model)
            }
            Msg::PageChanged(PageChanged::Board(BoardPageChange::IssueDragStopped(_))) => {
                crate::ws::issue::sync(model, orders);
            }
            Msg::PageChanged(PageChanged::Board(BoardPageChange::ChangePosition(
                issue_bellow_id,
            ))) => crate::ws::issue::change_position(issue_bellow_id, model),
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
    if rebuild_visible {
        let visible_issues = ProjectPage::visible_issues(
            crate::match_page!(model, Project),
            model.epics(),
            model.issue_statuses(),
            model.issues(),
            model.user(),
        );
        crate::match_page_mut!(model, Project).visible_issues = visible_issues;
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::Project(Box::new(ProjectPage::default()));
}
