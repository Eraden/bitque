use {
    crate::{
        model::{self, ModalType, Model, Page},
        shared::{
            find_issue, go_to_board,
            styled_confirm_modal::StyledConfirmModal,
            styled_modal::{StyledModal, Variant as ModalVariant},
            ToNode,
        },
        ws::send_ws_msg,
        FieldChange, FieldId, Msg, OperationKind, ResourceKind,
    },
    jirs_data::{TimeTracking, WsMsg},
    seed::{prelude::*, *},
};

pub mod issues;

pub fn update(msg: &Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ModalDropped => match model.modals.pop() {
            Some(ModalType::EditIssue(..)) | Some(ModalType::AddIssue(..)) => {
                go_to_board(orders);
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

        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, _) => {
            match model.page {
                Page::EditIssue(issue_id) if model.modals.is_empty() => {
                    push_edit_modal(issue_id, model, orders)
                }
                Page::AddIssue if model.modals.is_empty() => push_add_modal(model, orders),
                _ => (),
            }
        }

        Msg::ChangePage(Page::EditIssue(issue_id)) => push_edit_modal(*issue_id, model, orders),

        Msg::ChangePage(Page::AddIssue) => push_add_modal(model, orders),

        #[cfg(debug_assertions)]
        Msg::GlobalKeyDown { key, .. } if key.eq("#") => {
            model.modals.push(ModalType::DebugModal);
        }

        #[cfg(debug_assertions)]
        Msg::GlobalKeyDown { key, .. } if key.eq(">") => {
            orders.skip();
            log!(model);
        }
        Msg::GlobalKeyDown { .. } => {
            orders.skip();
        }

        _ => (),
    }

    use crate::modals::{issue_statuses_delete, issues_create, issues_edit};
    issues_create::update(msg, model, orders);
    issues_edit::update(msg, model, orders);
    issue_statuses_delete::update(msg, model, orders);
}

pub fn view(model: &model::Model) -> Node<Msg> {
    use crate::modals::{issue_statuses_delete, issues_create, issues_edit};
    let modals: Vec<Node<Msg>> = model
        .modals
        .iter()
        .map(|modal| match modal {
            ModalType::EditIssue(issue_id, modal) => {
                if let Some(_issue) = find_issue(model, *issue_id) {
                    let details = issues_edit::view(model, modal.as_ref());
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
            ModalType::DeleteIssueConfirm(_id) => crate::modals::issues_delete::view(model),
            ModalType::AddIssue(modal) => issues_create::view(model, modal),
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
            ModalType::TimeTracking(issue_id) => {
                crate::modals::time_tracking::view(model, *issue_id)
            }
            ModalType::DeleteIssueStatusModal(delete_issue_modal) => {
                issue_statuses_delete::view(model, delete_issue_modal.delete_id)
            }
            #[cfg(debug_assertions)]
            ModalType::DebugModal => crate::modals::debug::view(model),
        })
        .collect();
    section![id!["modals"], modals]
}

fn push_add_modal(model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::issues_create::Model;
    model.modals.push(ModalType::AddIssue(Box::new(Model {
        project_id: model.project.as_ref().map(|p| p.id),
        ..Model::default()
    })));
}

fn push_edit_modal(issue_id: i32, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
            Box::new(crate::modals::issues_edit::Model::new(
                issue,
                time_tracking_type,
            )),
        )
    };
    send_ws_msg(
        WsMsg::IssueCommentsLoad(issue_id),
        model.ws.as_ref(),
        orders,
    );
    model.modals.push(modal);
}
