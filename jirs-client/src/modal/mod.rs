use seed::{prelude::*, *};

use jirs_data::WsMsg;

use crate::api::send_ws_msg;
use crate::model::{AddIssueModal, EditIssueModal, ModalType, Model, Page};
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::{find_issue, ToNode};
use crate::{model, FieldChange, FieldId, Msg};

mod add_issue;
mod confirm_delete_issue;
mod issue_details;

pub fn update(msg: &Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ModalDropped => match model.modals.pop() {
            Some(ModalType::EditIssue(..)) => {
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
            model.modals.push(modal_type.clone());
        }

        Msg::WsMsg(jirs_data::WsMsg::ProjectIssuesLoaded(_issues)) => match model.page.clone() {
            Page::EditIssue(issue_id) if model.modals.is_empty() => {
                push_edit_modal(&issue_id, model)
            }
            _ => (),
        },

        Msg::ChangePage(Page::EditIssue(issue_id)) => {
            push_edit_modal(issue_id, model);
            send_ws_msg(WsMsg::IssueCommentsRequest(issue_id.clone()));
        }

        Msg::ChangePage(Page::AddIssue) => {
            let mut modal = AddIssueModal::default();
            modal.project_id = model.project.as_ref().map(|p| p.id);
            model.modals.push(ModalType::AddIssue(modal));
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
        })
        .collect();
    section![id!["modals"], modals]
}

fn push_edit_modal(issue_id: &i32, model: &mut Model) {
    let modal = {
        let issue = match find_issue(model, *issue_id) {
            Some(issue) => issue,
            _ => return,
        };
        ModalType::EditIssue(*issue_id, EditIssueModal::new(issue))
    };
    model.modals.push(modal);
}
