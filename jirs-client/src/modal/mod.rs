use seed::{prelude::*, *};

use jirs_data::{TimeTracking, WsMsg};

use crate::api::send_ws_msg;
use crate::model::{AddIssueModal, EditIssueModal, ModalType, Model, Page};
use crate::shared::styled_confirm_modal::StyledConfirmModal;
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::{find_issue, ToNode};
use crate::{model, FieldChange, FieldId, Msg};

mod add_issue;
mod confirm_delete_issue;
mod issue_details;
pub mod time_tracking;

pub fn update(msg: &Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ModalDropped => match model.modals.pop() {
            Some(ModalType::EditIssue(..)) | Some(ModalType::AddIssue(..)) => {
                seed::push_route(vec!["board"]);
                orders.send_msg(Msg::ChangePage(Page::Project));
            }
            _ => (),
        },

        Msg::ModalChanged(FieldChange::LinkCopied(FieldId::CopyButtonLabel, true)) => {
            for modal in model.modals.iter_mut() {
                if let ModalType::EditIssue(_, edit) = modal {
                    edit.link_copied = true;
                }
            }
        }

        Msg::ModalOpened(modal_type) => {
            model.modals.push(modal_type.as_ref().clone());
        }

        Msg::WsMsg(jirs_data::WsMsg::ProjectIssuesLoaded(_issues)) => match model.page {
            Page::EditIssue(issue_id) if model.modals.is_empty() => {
                push_edit_modal(issue_id, model)
            }
            _ => (),
        },

        Msg::ChangePage(Page::EditIssue(issue_id)) => {
            push_edit_modal(*issue_id, model);
        }

        Msg::ChangePage(Page::AddIssue) => {
            let mut modal = AddIssueModal::default();
            modal.project_id = model.project.as_ref().map(|p| p.id);
            model.modals.push(ModalType::AddIssue(Box::new(modal)));
        }

        _ => (),
    }
    add_issue::update(msg, model, orders);
    issue_details::update(msg, model, orders);
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let modals: Vec<Node<Msg>> = model
        .modals
        .iter()
        .map(|modal| match modal {
            ModalType::EditIssue(issue_id, modal) => {
                if let Some(_issue) = find_issue(model, *issue_id) {
                    let details = issue_details::view(model, &modal);
                    StyledModal::build()
                        .variant(ModalVariant::Center)
                        .width(1040)
                        .children(vec![details])
                        .build()
                        .into_node()
                } else {
                    empty![]
                }
            }
            ModalType::DeleteIssueConfirm(_id) => confirm_delete_issue::view(model),
            ModalType::AddIssue(modal) => add_issue::view(model, modal),
            ModalType::DeleteCommentConfirm(comment_id) => {
                let comment_id = *comment_id;
                StyledConfirmModal::build()
                    .title("Are you sure you want to delete this comment?")
                    .message("Once you delete, it's gone for good.")
                    .confirm_text("Delete comment")
                    .on_confirm(mouse_ev(Ev::Click, move |_| Msg::DeleteComment(comment_id)))
                    .build()
                    .into_node()
            }
            ModalType::TimeTracking(issue_id) => time_tracking::view(model, *issue_id),
        })
        .collect();
    section![id!["modals"], modals]
}

fn push_edit_modal(issue_id: i32, model: &mut Model) {
    let time_tracking_type = model
        .project
        .as_ref()
        .map(|p| p.time_tracking)
        .unwrap_or_else(|| TimeTracking::Untracked);
    let modal = {
        let issue = match find_issue(model, issue_id) {
            Some(issue) => issue,
            _ => return,
        };
        ModalType::EditIssue(
            issue_id,
            Box::new(EditIssueModal::new(issue, time_tracking_type)),
        )
    };
    send_ws_msg(WsMsg::IssueCommentsRequest(issue_id));
    model.modals.push(modal);
}
