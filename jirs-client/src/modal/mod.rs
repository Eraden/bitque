use seed::{prelude::*, *};

use jirs_data::{Issue, IssueType, UpdateIssuePayload};

use crate::api::send_ws_msg;
use crate::model::{AddIssueModal, EditIssueModal, ModalType, Page};
use crate::shared::styled_modal::{StyledModal, Variant as ModalVariant};
use crate::shared::styled_select::StyledSelectChange;
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

        Msg::ChangePage(Page::EditIssue(issue_id)) => {
            let value = find_issue(model, *issue_id)
                .map(|issue| issue.issue_type.clone())
                .unwrap_or_else(|| IssueType::Task);
            model.modals.push(ModalType::EditIssue(
                *issue_id,
                EditIssueModal {
                    id: *issue_id,
                    top_select_opened: false,
                    top_select_filter: "".to_string(),
                    value,
                    link_copied: false,
                },
            ));
        }
        Msg::ChangePage(Page::AddIssue) => {
            let mut modal = AddIssueModal::default();
            modal.project_id = model.project.as_ref().map(|p| p.id).unwrap_or_default();
            model.modals.push(ModalType::AddIssue(modal));
        }

        Msg::StyledSelectChanged(FieldId::IssueTypeEditModalTop, change) => {
            match (change, model.modals.last_mut()) {
                (StyledSelectChange::Text(ref text), Some(ModalType::EditIssue(_, modal))) => {
                    modal.top_select_filter = text.clone();
                }
                (
                    StyledSelectChange::DropDownVisibility(flag),
                    Some(ModalType::EditIssue(_, modal)),
                ) => {
                    modal.top_select_opened = *flag;
                }
                (
                    StyledSelectChange::Changed(value),
                    Some(ModalType::EditIssue(issue_id, modal)),
                ) => {
                    modal.value = (*value).into();
                    let mut found: Option<&mut Issue> = None;
                    for issue in model.issues.iter_mut() {
                        if issue.id == *issue_id {
                            found = Some(issue);
                            break;
                        }
                    }
                    let issue = match found {
                        Some(i) => i,
                        _ => return,
                    };

                    let form = UpdateIssuePayload {
                        title: Some(issue.title.clone()),
                        issue_type: Some(modal.value.clone()),
                        status: Some(issue.status.clone()),
                        priority: Some(issue.priority.clone()),
                        list_position: Some(issue.list_position),
                        description: Some(issue.description.clone()),
                        description_text: Some(issue.description_text.clone()),
                        estimate: Some(issue.estimate.clone()),
                        time_spent: Some(issue.time_spent.clone()),
                        time_remaining: Some(issue.time_remaining.clone()),
                        project_id: Some(issue.project_id.clone()),
                        user_ids: Some(issue.user_ids.clone()),
                    };
                    send_ws_msg(jirs_data::WsMsg::IssueUpdateRequest(issue_id.clone(), form));
                }
                _ => {}
            }
        }

        _ => (),
    }
    add_issue::update(msg, model, orders);
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let modals: Vec<Node<Msg>> = model
        .modals
        .iter()
        .map(|modal| match modal {
            ModalType::EditIssue(issue_id, modal) => {
                if let Some(issue) = find_issue(model, *issue_id) {
                    let details = issue_details::view(model, issue, &modal);
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
