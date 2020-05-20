use seed::prelude::*;

use jirs_data::{IssueStatusId, WsMsg};

use crate::model::{DeleteIssueStatusModal, ModalType, Model};
use crate::shared::styled_confirm_modal::StyledConfirmModal;
use crate::shared::ToNode;
use crate::{model, Msg, WebSocketChanged};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let _modal: &mut Box<DeleteIssueStatusModal> =
        match model.modals.iter_mut().find_map(|modal| match modal {
            ModalType::DeleteIssueStatusModal(modal) => Some(modal),
            _ => None,
        }) {
            Some(m) => m,
            _ => return,
        };

    match msg {
        Msg::DeleteIssueStatus(issue_status_id) => {
            crate::ws::send_ws_msg(
                WsMsg::IssueStatusDelete(*issue_status_id),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::IssueStatusDeleted(_))) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => (),
    };
}

pub fn view(_model: &model::Model, issue_status_id: IssueStatusId) -> Node<Msg> {
    StyledConfirmModal::build()
        .title("Delete column")
        .cancel_text("No")
        .confirm_text("Yes")
        .on_confirm(mouse_ev(Ev::Click, move |_| {
            Msg::DeleteIssueStatus(issue_status_id)
        }))
        .message("Are you sure you want to delete column?")
        .build()
        .into_node()
}
