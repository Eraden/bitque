use jirs_data::msg::WsMsgComment;
use jirs_data::{CommentId, EpicId, IssueId, IssueStatusId, TimeTracking, WsMsg};
use seed::prelude::*;

use crate::model::{ModalType, Model, Page};
use crate::shared::go_to_board;
use crate::ws::send_ws_msg;
use crate::{FieldChange, FieldId, Msg, OperationKind, ResourceKind};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum DebugMsg {
    Console,
    Modal,
}

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ModalDropped if !model.modal_stack().is_empty() => {
            drop_modal(model, orders);
        }

        Msg::ModalChanged(FieldChange::LinkCopied(FieldId::CopyButtonLabel, true)) => {
            if let Some(edit) = &mut model.modals_mut().edit_issue {
                edit.link_copied = true;
            }
        }

        Msg::ModalOpened(modal_type) => {
            push_modal(modal_type, model, orders);
        }

        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, _) => {
            match model.page {
                Page::EditIssue(issue_id) => push_edit_issue_modal(issue_id, model, orders),
                Page::AddIssue => push_add_issue_modal(model, orders),
                Page::DeleteEpic(id) => push_delete_epic_modal(id, model, orders),
                Page::EditEpic(id) => push_edit_epic_modal(id, model, orders),
                _ => (),
            }
        }

        Msg::ChangePage(Page::EditIssue(id)) => push_edit_issue_modal(*id, model, orders),
        Msg::ChangePage(Page::AddIssue) => push_add_issue_modal(model, orders),
        Msg::ChangePage(Page::DeleteEpic(id)) => push_delete_epic_modal(*id, model, orders),
        Msg::ChangePage(Page::EditEpic(id)) => push_edit_epic_modal(*id, model, orders),

        #[cfg(debug_assertions)]
        Msg::Debug(DebugMsg::Modal) => push_debug_modal(model),

        #[cfg(debug_assertions)]
        Msg::Debug(DebugMsg::Console) => {
            orders.skip();
            log::debug!("{:?}", model);
        }

        _ => (),
    }

    {
        use crate::modals::*;
        issues_create::update(msg, model, orders);
        issues_edit::update(msg, model, orders);
        issue_statuses_delete::update(msg, model, orders);
        epics_edit::update(msg, model, orders);
        epics_delete::update(msg, model, orders);
    }
}

// MODALS

fn push_modal(modal_type: &ModalType, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match modal_type {
        ModalType::AddIssue(_) => push_add_issue_modal(model, orders),
        ModalType::EditIssue(id) => {
            if let Some(id) = id {
                push_edit_issue_modal(*id, model, orders);
            }
        }
        ModalType::DeleteIssueConfirm(id) => {
            if let Some(id) = id {
                push_delete_issue_modal(*id, model, orders);
            }
        }
        ModalType::DeleteEpic(id) => {
            if let Some(id) = id {
                push_delete_epic_modal(*id, model, orders);
            }
        }
        ModalType::EditEpic(id) => {
            if let Some(id) = id {
                push_edit_epic_modal(*id, model, orders);
            }
        }
        ModalType::DeleteCommentConfirm(id) => {
            if let Some(id) = id {
                push_delete_comment_modal(*id, model, orders);
            }
        }
        ModalType::TimeTracking(id) => {
            if let Some(id) = id {
                push_time_track_modal(*id, model, orders);
            }
        }
        ModalType::DeleteIssueStatusModal(id) => {
            if let Some(id) = id {
                push_delete_issue_status_modal(*id, model, orders);
            }
        }
        #[cfg(debug_assertions)]
        ModalType::DebugModal(_) => push_debug_modal(model),
    }
}

fn drop_modal(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let modal = model.modal_stack_mut().pop().unwrap();
    let modals = model.modals_mut();
    match modal {
        ModalType::AddIssue(_) => {
            modals.add_issue = None;
        }
        ModalType::EditIssue(_) => {
            modals.edit_issue = None;
        }
        ModalType::DeleteIssueConfirm(_) => {
            modals.delete_issue_confirm = None;
        }
        ModalType::DeleteEpic(_) => {
            modals.delete_epic = None;
        }
        ModalType::EditEpic(_) => {
            modals.edit_epic = None;
        }
        ModalType::DeleteCommentConfirm(_) => {
            modals.delete_comment_confirm = None;
        }
        ModalType::TimeTracking(_) => {
            modals.time_tracking = None;
        }
        ModalType::DeleteIssueStatusModal(_) => {
            modals.delete_issue_status_modal = None;
        }

        #[cfg(debug_assertions)]
        ModalType::DebugModal(_) => {
            modals.debug_modal = None;
        }
    };
    match modal {
        ModalType::EditIssue(_)
        | ModalType::AddIssue(_)
        | ModalType::DeleteEpic(_)
        | ModalType::EditEpic(_) => {
            go_to_board(orders);
        }
        _ => (),
    }
}

// ISSUE

fn push_add_issue_modal(model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::issues_create::Model;
    if model.modals().add_issue.is_some() {
        return;
    }
    model.modal_stack_mut().push(ModalType::AddIssue(None));
    model.modals_mut().add_issue = Some(Model {
        project_id: model.project.as_ref().map(|p| p.id),
        ..Model::default()
    });
}

fn push_edit_issue_modal(issue_id: EpicId, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.modals().edit_issue.is_some() {
        return;
    }
    let time_tracking_type = model
        .project
        .as_ref()
        .map(|p| p.time_tracking)
        .unwrap_or(TimeTracking::Untracked);

    let modal = {
        let issue = match model.issues_by_id.get(&issue_id) {
            Some(issue) => issue,
            _ => return,
        };
        let user_mode = model
            .user_settings
            .as_ref()
            .map(|u| u.text_editor_mode)
            .unwrap_or_default();
        crate::modals::issues_edit::Model::new(user_mode, issue, time_tracking_type)
    };
    send_ws_msg(
        WsMsg::Comment(WsMsgComment::IssueCommentsLoad(issue_id)),
        model.ws.as_ref(),
        orders,
    );
    model
        .modal_stack_mut()
        .push(ModalType::EditIssue(Some(issue_id)));
    model.modals_mut().edit_issue = Some(modal);
}

fn push_delete_issue_modal(id: IssueId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    if model.modals().delete_issue_confirm.is_some() {
        return;
    }
    model
        .modal_stack_mut()
        .push(ModalType::DeleteIssueConfirm(Some(id)));
    model.modals_mut().delete_issue_confirm =
        Some(crate::modals::issues_delete::Model { issue_id: id });
}

// ISSUE STATUS

fn push_delete_issue_status_modal(
    id: IssueStatusId,
    model: &mut Model,
    _orders: &mut impl Orders<Msg>,
) {
    use crate::modals::issue_statuses_delete::Model;
    if model.modals().delete_issue_status_modal.is_some() {
        return;
    }
    let modal = Model::new(id);
    model
        .modal_stack_mut()
        .push(ModalType::DeleteIssueStatusModal(Some(id)));
    model.modals_mut().delete_issue_status_modal = Some(modal);
}

// EPIC

fn push_edit_epic_modal(id: EpicId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::epics_edit::Model;
    if model.modals().edit_epic.is_some() {
        return;
    }
    let modal = Model::new(id, model);
    model.modal_stack_mut().push(ModalType::EditEpic(Some(id)));
    model.modals_mut().edit_epic = Some(modal);
}

fn push_delete_epic_modal(id: EpicId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::epics_delete::Model;
    if model.modals_mut().delete_epic.is_some() {
        return;
    }
    model
        .modal_stack_mut()
        .push(ModalType::DeleteEpic(Some(id)));
    model.modals_mut().delete_epic = Some(Model::new(id, model));
}

// COMMENTS

fn push_delete_comment_modal(id: CommentId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::comments_delete::Model;
    if model.modals_mut().delete_comment_confirm.is_some() {
        return;
    }
    model
        .modal_stack_mut()
        .push(ModalType::DeleteCommentConfirm(Some(id)));
    model.modals_mut().delete_comment_confirm = Some(Model::new(id));
}

// TIME TRACK

fn push_time_track_modal(id: IssueId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::time_tracking::Model;
    if model.modals_mut().time_tracking.is_some() {
        return;
    }
    model
        .modal_stack_mut()
        .push(ModalType::TimeTracking(Some(id)));
    model.modals_mut().time_tracking = Some(Model::new(id));
}

// DEBUG

#[cfg(debug_assertions)]
fn push_debug_modal(model: &mut Model) {
    if model.modals().debug_modal.is_some() {
        return;
    }
    model.modal_stack_mut().push(ModalType::DebugModal(None));
    model.modals_mut().debug_modal = Some(true);
}
