use crate::api::update_issue;
use crate::model::{EditIssueModal, ModalType, Page};
use crate::project::issue_details;
use crate::shared::modal::{Modal, Variant as ModalVariant};
use crate::shared::styled_select::StyledSelectChange;
use crate::shared::{find_issue, ToNode};
use crate::{model, FieldId, Msg};
use jirs_data::{Issue, IssueType, UpdateIssuePayload};
use seed::{prelude::*, *};

pub fn update(msg: &Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::PopModal => match model.modals.pop() {
            _ => (),
        },
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
                    let project = match model.project.as_mut() {
                        Some(p) => p,
                        _ => return,
                    };
                    let mut found: Option<&mut Issue> = None;
                    for issue in project.issues.iter_mut() {
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
                    orders.skip().perform_cmd(update_issue(
                        model.host_url.clone(),
                        *issue_id,
                        form,
                    ));
                }
                _ => {}
            }
        }
        _ => (),
    }
}

pub fn view(model: &model::Model) -> Node<Msg> {
    let modals: Vec<Node<Msg>> = model
        .modals
        .iter()
        .map(|modal| match modal {
            ModalType::EditIssue(issue_id, modal) => {
                if let Some(issue) = find_issue(model, *issue_id) {
                    let details = issue_details(model, issue, &modal);
                    let modal = Modal {
                        variant: ModalVariant::Center,
                        width: 1040,
                        with_icon: false,
                        children: vec![details],
                    }
                    .into_node();
                    modal
                } else {
                    empty![]
                }
            }
            _ => empty![],
        })
        .collect();
    section![id!["modals"], modals]
}
