use seed::{prelude::*, *};

use crate::shared::styled_modal::StyledModal;
use crate::shared::ToNode;
use crate::{model, Msg};

pub fn view(model: &model::Model) -> Node<Msg> {
    let handle_issue_delete = mouse_ev(Ev::Click, |_| Msg::NoOp);
    StyledModal::build()
        .title("Are you sure you want to delete this issue?")
        .message("Once you delete, it's gone for good.")
        .confirm_text("Delete issue")
        .on_confirm(handle_issue_delete)
        .build()
        .into_node()
}
