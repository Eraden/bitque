use {
    crate::{
        components::{
            styled_avatar::StyledAvatar, styled_button::StyledButton, styled_editor::StyledEditor,
            styled_field::StyledField, styled_icon::Icon, styled_input::StyledInput,
            styled_modal::*, styled_select::StyledSelect,
        },
        modals::{
            epic_field, issues_edit::Model as EditIssueModal, time_tracking::time_tracking_field,
        },
        model::{ModalType, Model},
        shared::{tracking_widget::tracking_link, IntoChild, ToChild, ToNode},
        EditIssueModalSection, FieldChange, FieldId, Msg,
    },
    comments::*,
    jirs_data::{CommentFieldId, IssueFieldId, IssuePriority, IssueType, TimeTracking},
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_icon::StyledIcon;
use crate::components::styled_select::SelectVariant;
use crate::components::styled_select_child::StyledSelectChildBuilder;

mod comments;

pub fn view(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    model
        .issues_by_id
        .get(&modal.id)
        .map(|_issue| {
            StyledModal::centered_with_width_and_body(1040, vec![details(model, modal)]).into_node()
        })
        .unwrap_or(Node::Empty)
}

pub fn details(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    div![
        C!["issueDetails"],
        modal_header(model, modal),
        div![
            C!["content"],
            left_modal_column(model, modal),
            right_modal_column(model, modal),
        ],
    ]
}

fn modal_header(_model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        id,
        payload,
        top_type_state,
        link_copied,
        ..
    } = modal;

    let issue_id = *id;

    let click_handler = mouse_ev(Ev::Click, move |_| {
        let proto = seed::window().location().protocol().unwrap_or_default();
        let hostname = seed::window().location().hostname().unwrap_or_default();
        let link = format!(
            "{proto}//{hostname}/issues/{id}",
            proto = proto,
            hostname = hostname,
            id = issue_id
        );
        let el = match seed::html_document().create_element("textarea") {
            Ok(el) => el
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .clone(),
            _ => return None as Option<Msg>,
        };
        seed::body().append_child(&el).unwrap();
        el.set_text_content(Some(link.as_str()));
        el.select();
        el.set_selection_range(0, 9999).unwrap();
        seed::html_document().exec_command("copy").unwrap();
        seed::body().remove_child(&el).unwrap();
        Some(Msg::ModalChanged(FieldChange::LinkCopied(
            FieldId::CopyButtonLabel,
            true,
        )))
    });
    let close_handler = mouse_ev(Ev::Click, |ev| {
        ev.prevent_default();
        ev.stop_propagation();
        Msg::ModalDropped
    });
    let delete_confirmation_handler = mouse_ev(Ev::Click, move |_| {
        Msg::ModalOpened(ModalType::DeleteIssueConfirm(Some(issue_id)))
    });

    let copy_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(Icon::Link.into_node()),
        on_click: Some(click_handler),
        children: vec![span![if *link_copied {
            "Link Copied"
        } else {
            "Copy link"
        }]],
        ..Default::default()
    }
    .into_node();
    let delete_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(
            StyledIcon {
                icon: Icon::Trash,
                size: Some(19),
                ..Default::default()
            }
            .into_node(),
        ),
        on_click: Some(delete_confirmation_handler),
        ..Default::default()
    }
    .into_node();
    let close_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(
            StyledIcon {
                icon: Icon::Close,
                size: Some(24),
                ..Default::default()
            }
            .into_node(),
        ),
        on_click: Some(close_handler),
        ..Default::default()
    }
    .into_node();

    let issue_type_select = {
        let id = modal.id;
        let issue_type = &payload.issue_type;
        let text = format!("{} - {}", issue_type, id);

        StyledSelect {
            id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
            name: "type",
            text_filter: top_type_state.text_filter.as_str(),
            dropdown_width: Some(150),
            valid: true,
            opened: top_type_state.opened,
            options: Some(
                IssueType::default()
                    .into_iter()
                    .map(|t| t.into_child().name("type")),
            ),
            selected: vec![{
                let name = payload.issue_type.to_label();

                let type_icon = StyledIcon {
                    icon: payload.issue_type.clone().into(),
                    class_list: name,
                    ..Default::default()
                }
                .into_node();

                StyledSelectChildBuilder {
                    class_list: name,
                    text: Some(&text),
                    icon: Some(type_icon),
                    value: payload.issue_type.into(),
                    name: Some("type"),
                    ..Default::default()
                }
            }],
            ..Default::default()
        }
        .into_node()
    };

    div![
        C!["topActions"],
        issue_type_select,
        div![
            C!["topActionsRight"],
            copy_button,
            delete_button,
            close_button
        ],
    ]
}

fn left_modal_column(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        payload,
        description_state,
        comment_form,
        ..
    } = modal;

    let title = StyledInput {
        input_class_list: "issueSummary",
        wrapper_class_list: "issueSummary textarea",
        value: modal.title_state.value.as_str(),
        valid: modal.title_state.is_valid(),
        id: Some(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Title,
        ))),
        ..Default::default()
    }
    .into_node();

    let description = {
        StyledEditor {
            id: Some(FieldId::EditIssueModal(EditIssueModalSection::Issue(
                IssueFieldId::Description,
            ))),
            initial_text: description_state.initial_text.as_str(),
            text: description_state.initial_text.as_str(),
            html: payload.description.as_deref().unwrap_or_default(),
            mode: description_state.mode.clone(),
            update_event: Ev::Change,
        }
        .into_node()
    };
    let description_field = StyledField::build().input(description).build().into_node();

    let user_avatar = StyledAvatar {
        avatar_url: model.user.as_ref().and_then(|u| u.avatar_url.as_deref()),
        size: 32,
        class_list: "userAvatar",
        ..StyledAvatar::default()
    }
    .into_node();

    let create_comment = if comment_form.creating && comment_form.id.is_none() {
        build_comment_form(comment_form)
    } else {
        let creating_comment = comment_form.creating;
        let handler = mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            Msg::ModalChanged(FieldChange::ToggleCommentForm(
                FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                !creating_comment,
            ))
        });
        vec![div![C!["fakeTextArea"], "Add a comment...", handler]]
    };

    let comments = model.comments.iter().flat_map(|c| comment(model, modal, c));

    div![
        C!["left"],
        title,
        description_field,
        div![
            C!["comments"],
            div![C!["title"], "Comments"],
            div![
                C!["create"],
                user_avatar,
                div![
                    C!["right"],
                    create_comment,
                    div![
                        C!["proTip"],
                        strong![C!["strong"], "Pro tip: "],
                        "press ",
                        span![C!["tipLetter"], "M"],
                        " to comment"
                    ]
                ]
            ],
            comments
        ],
    ]
}

fn right_modal_column(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        payload,
        status_state,
        reporter_state,
        assignees_state,
        priority_state,
        ..
    } = modal;

    let status = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::IssueStatusId)),
        name: "status",
        opened: status_state.opened,
        variant: SelectVariant::Normal,
        text_filter: status_state.text_filter.as_str(),
        options: Some(
            model
                .issue_statuses
                .iter()
                .map(|opt| opt.to_child().name("status")),
        ),
        selected: model
            .issue_statuses
            .iter()
            .filter(|is| is.id == payload.issue_status_id)
            .map(|is| is.to_child().name("status"))
            .collect(),

        valid: true,
        ..Default::default()
    }
    .into_node();
    let status_field = StyledField::build()
        .input(status)
        .label("Status")
        .build()
        .into_node();

    let assignees = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
        name: "assignees",
        variant: SelectVariant::Empty,
        is_multi: true,
        opened: assignees_state.opened,
        text_filter: assignees_state.text_filter.as_str(),
        options: Some(
            model
                .users
                .iter()
                .map(|user| user.to_child().name("assignees")),
        ),
        selected: model
            .users
            .iter()
            .filter(|user| payload.user_ids.contains(&user.id))
            .map(|user| user.to_child().name("assignees"))
            .collect(),
        ..Default::default()
    }
    .into_node();
    let assignees_field = StyledField::build()
        .input(assignees)
        .label("Assignees")
        .build()
        .into_node();

    let reporter = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Reporter)),
        name: "reporter",
        opened: reporter_state.opened,
        variant: SelectVariant::Empty,
        text_filter: reporter_state.text_filter.as_str(),
        options: Some(
            model
                .users
                .iter()
                .map(|user| user.to_child().name("reporter")),
        ),
        selected: model
            .users
            .iter()
            .filter(|user| payload.reporter_id == user.id)
            .map(|user| user.to_child().name("reporter"))
            .collect(),
        ..Default::default()
    }
    .into_node();
    let reporter_field = StyledField::build()
        .input(reporter)
        .label("Reporter")
        .build()
        .into_node();

    let priority = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Priority)),
        name: "priority",
        variant: SelectVariant::Empty,
        opened: priority_state.opened,
        text_filter: priority_state.text_filter.as_str(),
        options: Some(
            IssuePriority::default()
                .into_iter()
                .map(|p| p.into_child().name("priority")),
        ),
        selected: vec![payload.priority.into_child().name("priority")],
        ..Default::default()
    }
    .into_node();
    let priority_field = StyledField::build()
        .input(priority)
        .label("Priority")
        .build()
        .into_node();

    let time_tracking_type = model
        .project
        .as_ref()
        .map(|p| p.time_tracking)
        .unwrap_or_else(|| TimeTracking::Untracked);

    let (estimate_field, tracking_field) = if time_tracking_type != TimeTracking::Untracked {
        let estimate_field = time_tracking_field(
            time_tracking_type,
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
            "Original Estimate (hours)",
            &modal.estimate,
            &modal.estimate_select,
        );

        let tracking = tracking_link(model, modal);
        let tracking_field = StyledField::build()
            .label("TIME TRACKING")
            .tip("")
            .input(tracking)
            .build()
            .into_node();
        (estimate_field, tracking_field)
    } else {
        (Node::Empty, Node::Empty)
    };

    let epic_field = epic_field(
        model,
        modal,
        FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::EpicName)),
    )
    .unwrap_or(Node::Empty);

    div![
        C!["right"],
        status_field,
        assignees_field,
        reporter_field,
        priority_field,
        estimate_field,
        tracking_field,
        epic_field,
    ]
}
