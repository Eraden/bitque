use {
    crate::{
        model::{ModalType, Model, Page},
        shared::go_to_board,
        ws::send_ws_msg,
        FieldChange, FieldId, Msg, OperationKind, ResourceKind,
    },
    jirs_data::{EpicId, IssueId, TimeTracking, WsMsg},
    seed::{prelude::*, *},
};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
        Msg::ChangePage(Page::DeleteEpic(issue_id)) => {
            push_delete_epic_modal(*issue_id, model, orders)
        }
        Msg::ChangePage(Page::EditEpic(issue_id)) => push_edit_epic_modal(*issue_id, model, orders),

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

    {
        use crate::modals::*;
        issues_create::update(msg, model, orders);
        issues_edit::update(msg, model, orders);
        issue_statuses_delete::update(msg, model, orders);
        epics_edit::update(msg, model, orders);
        epics_delete::update(msg, model, orders);
    }
}

fn push_add_modal(model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::issues_create::Model;
    model.modals.push(ModalType::AddIssue(Box::new(Model {
        project_id: model.project.as_ref().map(|p| p.id),
        ..Model::default()
    })));
}

fn push_edit_modal(issue_id: EpicId, model: &mut Model, orders: &mut impl Orders<Msg>) {
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

fn push_edit_epic_modal(epic_id: EpicId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::epics_edit::Model;
    let modal = Model::new(epic_id, model);
    model.modals.push(ModalType::EditEpic(Box::new(modal)));
}

fn push_delete_epic_modal(issue_id: IssueId, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use crate::modals::epics_delete::Model;
    let modal = Model::new(issue_id, model);
    model.modals.push(ModalType::DeleteEpic(Box::new(modal)));
}
