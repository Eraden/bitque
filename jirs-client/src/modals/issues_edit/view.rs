use {
    crate::{
        modals::{
            epic_field, issues_edit::Model as EditIssueModal, time_tracking::time_tracking_field,
        },
        model::{ModalType, Model},
        shared::{
            styled_avatar::StyledAvatar, styled_button::StyledButton, styled_editor::StyledEditor,
            styled_field::StyledField, styled_icon::Icon, styled_input::StyledInput,
            styled_select::StyledSelect, tracking_widget::tracking_link, IntoChild, ToChild,
            ToNode,
        },
        EditIssueModalSection, FieldChange, FieldId, Msg,
    },
    comments::*,
    jirs_data::{CommentFieldId, IssueFieldId, IssuePriority, IssueType, TimeTracking},
    seed::{prelude::*, *},
};

mod comments;

pub fn view(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
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
        seed::Url::new().add_path_part("board").go_and_push();

        Msg::ModalDropped
    });
    let delete_confirmation_handler = mouse_ev(Ev::Click, move |_| {
        Msg::ModalOpened(Box::new(ModalType::DeleteIssueConfirm(issue_id)))
    });

    let copy_button = StyledButton::build()
        .empty()
        .icon(Icon::Link)
        .on_click(click_handler)
        .children(vec![span![if *link_copied {
            "Link Copied"
        } else {
            "Copy link"
        }]])
        .build()
        .into_node();
    let delete_button = StyledButton::build()
        .empty()
        .icon(Icon::Trash.into_styled_builder().size(19).build())
        .on_click(delete_confirmation_handler)
        .build()
        .into_node();
    let close_button = StyledButton::build()
        .empty()
        .icon(Icon::Close.into_styled_builder().size(24).build())
        .on_click(close_handler)
        .build()
        .into_node();

    let issue_type_select = StyledSelect::build()
        .dropdown_width(150)
        .name("type")
        .text_filter(top_type_state.text_filter.as_str())
        .opened(top_type_state.opened)
        .valid(true)
        .options(
            IssueType::default()
                .into_iter()
                .map(|t| t.into_child().name("type")),
        )
        .selected(vec![{
            let id = modal.id;
            let issue_type = &payload.issue_type;
            issue_type
                .into_child()
                .name("type")
                .text_owned(format!("{} - {}", issue_type, id))
        }])
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Type,
        )))
        .into_node();

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

    let title = StyledInput::build()
        .add_input_class("issueSummary")
        .add_wrapper_class("issueSummary")
        .add_wrapper_class("textarea")
        .state(&modal.title_state)
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Title,
        )))
        .into_node();

    let description = {
        StyledEditor::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Description,
        )))
        .initial_text(description_state.initial_text.as_str())
        .html(payload.description.as_ref().cloned().unwrap_or_default())
        .mode(description_state.mode.clone())
        .update_on(Ev::Change)
        .build()
        .into_node()
    };
    let description_field = StyledField::build().input(description).build().into_node();

    let user_avatar = StyledAvatar::build()
        .add_class("userAvatar")
        .size(32)
        .avatar_url(
            model
                .user
                .as_ref()
                .and_then(|u| u.avatar_url.as_deref())
                .unwrap_or_default(),
        )
        .build()
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

    let comments: Vec<Node<Msg>> = model
        .comments
        .iter()
        .flat_map(|c| comment(model, modal, c))
        .collect();

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

    let status = StyledSelect::build()
        .name("status")
        .opened(status_state.opened)
        .normal()
        .text_filter(status_state.text_filter.as_str())
        .options(
            model
                .issue_statuses
                .iter()
                .map(|opt| opt.to_child().name("status")),
        )
        .selected(
            model
                .issue_statuses
                .iter()
                .filter(|is| is.id == payload.issue_status_id)
                .map(|is| is.to_child().name("status"))
                .collect(),
        )
        .valid(true)
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::IssueStatusId,
        )))
        .into_node();
    let status_field = StyledField::build()
        .input(status)
        .label("Status")
        .build()
        .into_node();

    let assignees = StyledSelect::build()
        .name("assignees")
        .opened(assignees_state.opened)
        .empty()
        .multi()
        .text_filter(assignees_state.text_filter.as_str())
        .options(
            model
                .users
                .iter()
                .map(|user| user.to_child().name("assignees")),
        )
        .selected(
            model
                .users
                .iter()
                .filter(|user| payload.user_ids.contains(&user.id))
                .map(|user| user.to_child().name("assignees"))
                .collect(),
        )
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Assignees,
        )))
        .into_node();
    let assignees_field = StyledField::build()
        .input(assignees)
        .label("Assignees")
        .build()
        .into_node();

    let reporter = StyledSelect::build()
        .name("reporter")
        .opened(reporter_state.opened)
        .empty()
        .text_filter(reporter_state.text_filter.as_str())
        .options(
            model
                .users
                .iter()
                .map(|user| user.to_child().name("reporter")),
        )
        .selected(
            model
                .users
                .iter()
                .filter(|user| payload.reporter_id == user.id)
                .map(|user| user.to_child().name("reporter"))
                .collect(),
        )
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Reporter,
        )))
        .into_node();
    let reporter_field = StyledField::build()
        .input(reporter)
        .label("Reporter")
        .build()
        .into_node();

    let priority = StyledSelect::build()
        .name("priority")
        .opened(priority_state.opened)
        .empty()
        .text_filter(priority_state.text_filter.as_str())
        .options(
            IssuePriority::default()
                .into_iter()
                .map(|p| p.into_child().name("priority")),
        )
        .selected(vec![payload.priority.into_child().name("priority")])
        .build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
            IssueFieldId::Priority,
        )))
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
