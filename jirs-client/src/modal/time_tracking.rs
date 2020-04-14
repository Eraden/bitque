use seed::{prelude::*, *};

use jirs_data::IssueId;

use crate::model::Model;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_modal::StyledModal;
use crate::shared::{find_issue, ToNode};
use crate::Msg;

pub fn view(model: &Model, issue_id: IssueId) -> Node<Msg> {
    let issue = match find_issue(model, issue_id) {
        Some(issue) => issue,
        _ => return empty![],
    };

    let modal_title = div![class!["modalTitle"], "Time tracking"];

    // let time_spent = StyledInput::build()

    let inputs = div![class!["inputs"], ""];

    StyledModal::build()
        .add_class("timeTrackingModal")
        .children(vec![modal_title, inputs])
        .build()
        .into_node()
}
