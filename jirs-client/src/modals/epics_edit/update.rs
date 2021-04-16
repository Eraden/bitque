use jirs_data::{EpicFieldId, IssueType, WsMsg};
use seed::prelude::*;

use crate::{send_ws_msg, FieldId, Msg, OperationKind, ResourceKind};

pub fn update(msg: &Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    let modal = match &mut model.modals.edit_epic {
        Some(modal) => modal,
        _ => return,
    };
    modal.update(msg, orders);
    match msg {
        Msg::ResourceChanged(
            ResourceKind::Epic,
            OperationKind::SingleLoaded | OperationKind::SingleModified,
            Some(id),
        ) => {
            let name = model
                .epics_by_id
                .get(id)
                .map(|epic| epic.name.as_str())
                .unwrap_or_default();
            modal.name.value = name.to_string();
        }
        Msg::ResourceChanged(ResourceKind::Epic, OperationKind::ListLoaded, None) => {
            let epic_id = modal.epic_id;
            let name = model
                .epics_by_id
                .get(&epic_id)
                .map(|epic| epic.name.as_str())
                .unwrap_or_default();
            modal.name.value = name.to_string();
        }
        Msg::StrInputChanged(FieldId::EditEpic(EpicFieldId::Name), s) => {
            let epic_id = modal.epic_id;
            send_ws_msg(
                WsMsg::EpicUpdate(epic_id, s.to_string()),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::TransformEpic => {
            let epic_id = modal.epic_id;
            let issue_type: IssueType = modal.transform_into.value.into();
            send_ws_msg(
                WsMsg::EpicTransform(epic_id, issue_type),
                model.ws.as_ref(),
                orders,
            );
            orders.skip().send_msg(Msg::ModalDropped);
        }
        _ => (),
    };
}
