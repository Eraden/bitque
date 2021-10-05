use jirs_data::msg::WsMsgIssueStatus;
use jirs_data::WsMsg;
use seed::prelude::*;

use crate::model::Model;
use crate::{Msg, OperationKind, ResourceKind};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let _modal = match &mut model.modals_mut().delete_issue_status_modal {
        Some(m) => m,
        _ => return,
    };

    match msg {
        Msg::DeleteIssueStatus(issue_status_id) => {
            crate::ws::send_ws_msg(
                WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusDelete(*issue_status_id)),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::ResourceChanged(ResourceKind::IssueStatus, OperationKind::SingleRemoved, Some(_)) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => (),
    };
}
