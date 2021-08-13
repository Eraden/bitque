use jirs_data::msg::{WsMsgEpic, WsMsgInvitation, WsMsgIssueStatus, WsMsgProject};
use jirs_data::WsMsg;
use seed::app::Orders;

use crate::model::Model;
use crate::ws::enqueue_ws_msg;
use crate::Msg;

pub fn board_load(model: &mut Model, orders: &mut impl Orders<Msg>) {
    enqueue_ws_msg(
        vec![
            WsMsg::IssueStatus(WsMsgIssueStatus::IssueStatusesLoad),
            WsMsg::Project(WsMsgProject::ProjectIssuesLoad),
            WsMsg::Project(WsMsgProject::ProjectUsersLoad),
            WsMsg::Epic(WsMsgEpic::EpicsLoad),
        ],
        model.ws.as_ref(),
        orders,
    );
}

pub fn invitation_load(model: &mut Model, orders: &mut impl Orders<Msg>) {
    enqueue_ws_msg(
        vec![
            WsMsg::Invitation(WsMsgInvitation::InvitationListLoad),
            WsMsg::Invitation(WsMsgInvitation::InvitedUsersLoad),
        ],
        model.ws.as_ref(),
        orders,
    );
}
