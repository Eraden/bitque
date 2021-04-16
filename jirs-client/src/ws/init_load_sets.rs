use jirs_data::WsMsg;
use seed::app::Orders;

use crate::model::Model;
use crate::ws::enqueue_ws_msg;
use crate::Msg;

pub fn board_load(model: &mut Model, orders: &mut impl Orders<Msg>) {
    enqueue_ws_msg(
        vec![
            WsMsg::IssueStatusesLoad,
            WsMsg::ProjectIssuesLoad,
            WsMsg::ProjectUsersLoad,
            WsMsg::EpicsLoad,
        ],
        model.ws.as_ref(),
        orders,
    );
}

pub fn invitation_load(model: &mut Model, orders: &mut impl Orders<Msg>) {
    enqueue_ws_msg(
        vec![WsMsg::InvitationListLoad, WsMsg::InvitedUsersLoad],
        model.ws.as_ref(),
        orders,
    );
}
