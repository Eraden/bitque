use {
    crate::{
        components::{styled_button::*, styled_confirm_modal::*, styled_icon::*, styled_modal::*},
        modals::epic_delete::Model,
        model,
        shared::ToNode,
        Msg,
    },
    seed::{prelude::*, *},
};

pub fn view(model: &model::Model, modal: &Model) -> Node<Msg> {
    if modal.related_issues.is_empty() {
        StyledConfirmModal::build()
            .title("Delete empty epic")
            .cancel_text("Cancel")
            .confirm_text("Delete epic")
            .on_confirm(mouse_ev("click", move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Msg::DeleteEpic
            }))
            .build()
            .into_node()
    } else {
        StyledModal::build()
            .add_class("deleteEpic")
            .center()
            .width(600)
            .child(warning(model, modal))
            .build()
            .into_node()
    }
}

fn warning(model: &model::Model, modal: &Model) -> Node<Msg> {
    let issues: Vec<Node<Msg>> = modal
        .related_issues
        .iter()
        .flat_map(|id| model.issues_by_id.get(id))
        .map(|issue| {
            let link = StyledIcon::build(Icon::Link).build().into_node();
            li![div![
                C!["relatedIssue"],
                a![
                    attrs! {"href" => format!("/issues/{}", issue.id)},
                    link,
                    issue.title.as_str()
                ]
            ]]
        })
        .collect();

    let close = StyledButton::build()
        .text("Close")
        .on_click(mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::ModalDropped
        }))
        .secondary()
        .build()
        .into_node();

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
