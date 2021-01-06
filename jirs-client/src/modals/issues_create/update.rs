use {
    crate::{
        model::{IssueModal, ModalType},
        shared::styled_select::StyledSelectChanged,
        ws::send_ws_msg,
        FieldId, Msg, WebSocketChanged,
    },
    jirs_data::{IssueFieldId, UserId, WsMsg},
    seed::prelude::*,
};

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = model
        .modals
        .iter_mut()
        .find(|modal| matches!(modal, ModalType::AddIssue(..)));
    let modal = match modal {
        Some(ModalType::AddIssue(modal)) => modal,
        _ => return,
    };

    modal.update_states(msg, orders);

    match msg {
        Msg::AddEpic => {
            send_ws_msg(
                WsMsg::EpicCreate(modal.title_state.value.clone()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::AddIssue => {
            let user_id = model.user.as_ref().map(|u| u.id).unwrap_or_default();
            let project_id = model.project.as_ref().map(|p| p.id).unwrap_or_default();
            let type_value = modal.type_state.values.get(0).cloned().unwrap_or_default();
            match type_value {
                0 | 1 | 2 => {
                    let issue_type = type_value.into();
                    let payload = jirs_data::CreateIssuePayload {
                        title: modal.title_state.value.clone(),
                        issue_type,
                        issue_status_id: modal.issue_status_id,
                        priority: modal.priority,
                        description: modal.description.clone(),
                        description_text: modal.description_text.clone(),
                        estimate: modal.estimate,
                        time_spent: modal.time_spent,
                        time_remaining: modal.time_remaining,
                        project_id: modal.project_id.unwrap_or(project_id),
                        user_ids: modal.user_ids.clone(),
                        reporter_id: modal.reporter_id.unwrap_or_else(|| user_id),
                        epic_id: modal.epic_id,
                    };

                    send_ws_msg(
                        jirs_data::WsMsg::IssueCreate(payload),
                        model.ws.as_ref(),
                        orders,
                    );
                }
                _ => {
                    //
                }
            };
        }

        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueCreated(issue))) => {
            model.issues.push(issue.clone());
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::EpicCreated(_))) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }

        Msg::StrInputChanged(FieldId::AddIssueModal(IssueFieldId::Description), value) => {
            modal.description = Some(value.clone());
        }

        // ReporterAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Reporter),
            StyledSelectChanged::Changed(id),
        ) => {
            modal.reporter_id = id.map(|n| n as UserId);
        }

        // AssigneesAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChanged::Changed(Some(id)),
        ) => {
            let id = *id as UserId;
            if !modal.user_ids.contains(&id) {
                modal.user_ids.push(id);
            }
        }
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Assignees),
            StyledSelectChanged::RemoveMulti(id),
        ) => {
            let id = *id as i32;
            let mut old: Vec<i32> = vec![];
            std::mem::swap(&mut old, &mut modal.user_ids);
            for user_id in old {
                if id != user_id {
                    modal.user_ids.push(user_id);
                }
            }
        }

        // IssuePriorityAddIssueModal
        Msg::StyledSelectChanged(
            FieldId::AddIssueModal(IssueFieldId::Priority),
            StyledSelectChanged::Changed(Some(id)),
        ) => {
            modal.priority = (*id).into();
        }

        _ => (),
    }
}
