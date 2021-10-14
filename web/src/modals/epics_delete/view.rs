use seed::prelude::*;
use seed::*;

use crate::components::styled_button::*;
use crate::components::styled_confirm_modal::*;
use crate::components::styled_icon::*;
use crate::components::styled_modal::*;
use crate::modals::epics_delete::Model;
use crate::{model, Msg};

pub fn view(model: &model::Model, modal: &Model) -> Node<Msg> {
    if modal.related_issues.is_empty() {
        StyledConfirmModal {
            title: "Delete empty epic",
            confirm_text: "Delete epic",
            cancel_text: "Cancel",
            on_confirm: Some(mouse_ev("click", move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Msg::DeleteEpic
            })),
            ..Default::default()
        }
        .render()
    } else {
        StyledModal {
            children: vec![warning(model, modal)],
            width: Some(600),
            class_list: "deleteEpic",
            ..Default::default()
        }
        .render()
    }
}

fn warning(model: &model::Model, modal: &Model) -> Node<Msg> {
    let issues = modal
        .related_issues
        .iter()
        .flat_map(|id| model.issues_by_id.get(id))
        .map(|issue| {
            let link = StyledIcon::from(Icon::Link).render();
            li![div![
                C!["relatedIssue"],
                a![
                    attrs! {"href" => format!("/issues/{}", issue.id)},
                    link,
                    issue.title.as_str()
                ]
            ]]
        });

    let close = StyledButton {
        text: Some("Close"),
        on_click: Some(mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::ModalDropped
        })),
        variant: ButtonVariant::Secondary,
        ..Default::default()
    }
    .render();

    section![
        h3![C!["header"], "Cannot delete epic"],
        div![
            C!["warning"],
            "This epic have related issues. Please move or delete them first."
        ],
        ol![C!["relatedList"], issues],
        div![C!["actions"], close]
    ]
}
