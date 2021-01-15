use {
    crate::{
        modals::issue_statuses_delete::Model as DeleteIssueStatusModal,
        model::{ModalType, Model},
        Msg, OperationKind, ResourceKind,
    },
    jirs_data::WsMsg,
    seed::prelude::*,
};

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
        Msg::ResourceChanged(ResourceKind::IssueStatus, OperationKind::SingleRemoved, Some(_)) => {
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => (),
    };
}
