use comments::*;
use jirs_data::{
    CommentFieldId, IssueFieldId, IssuePriority, IssueStatus, IssueType, TimeTracking,
    UpdateIssuePayload, User,
};
use seed::prelude::*;
use seed::*;

use crate::components::styled_avatar::StyledAvatar;
use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_editor::render_styled_editor;
use crate::components::styled_field::StyledField;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::StyledInput;
use crate::components::styled_modal::*;
use crate::components::styled_select::{SelectVariant, StyledSelect, StyledSelectState};
use crate::components::styled_select_child::StyledSelectOption;
use crate::components::styled_tip::styled_tip;
use crate::modals::epic_field;
use crate::modals::issues_edit::Model as EditIssueModal;
use crate::modals::time_tracking::time_tracking_field;
use crate::model::{ModalType, Model};
use crate::shared::tracking_widget::tracking_link;
use crate::{BuildMsg, EditIssueModalSection, FieldChange, FieldId, Msg};

mod comments;

#[inline(always)]
pub fn view(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    model
        .issues_by_id
        .get(&modal.id)
        .map(|_issue| {
            StyledModal {
                variant: ModalVariant::Center,
                width: Some(1014),
                with_icon: false,
                children: vec![details(model, modal)],
                class_list: "",
            }
            .render()
        })
        .unwrap_or(Node::Empty)
}

#[inline(always)]
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

#[inline(always)]
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
        icon: Some(StyledIcon::from(Icon::Link).render()),
        on_click: Some(click_handler),
        children: vec![span![if *link_copied {
            "Link Copied"
        } else {
            "Copy link"
        }]],
        ..Default::default()
    }
    .render();
    let delete_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(
            StyledIcon {
                icon: Icon::Trash,
                size: Some(19),
                ..Default::default()
            }
            .render(),
        ),
        on_click: Some(delete_confirmation_handler),
        ..Default::default()
    }
    .render();
    let close_button = StyledButton {
        variant: ButtonVariant::Empty,
        icon: Some(
            StyledIcon {
                icon: Icon::Close,
                size: Some(24),
                ..Default::default()
            }
            .render(),
        ),
        on_click: Some(close_handler),
        ..Default::default()
    }
    .render();

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
                    .map(|t| type_select_option(t, &text)),
            ),
            selected: vec![type_select_option(payload.issue_type, &text)],
            ..Default::default()
        }
        .render()
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

#[inline(always)]
fn type_select_option<'l>(t: IssueType, text: &'l str) -> StyledSelectOption<'l> {
    let name = t.to_label();
    StyledSelectOption {
        class_list: name,
        text: Some(text),
        icon: Some(
            StyledIcon {
                icon: t.into(),
                class_list: name,
                ..Default::default()
            }
            .render(),
        ),
        value: t.into(),
        name: Some("type"),
        ..Default::default()
    }
}

#[inline(always)]
fn left_modal_column(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
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
    .render();

    let description = render_styled_editor(&description_state);
    let description_field = StyledField {
        input: description,
        ..Default::default()
    }
    .render();

    let user_avatar = StyledAvatar {
        avatar_url: model.user.as_ref().and_then(|u| u.avatar_url.as_deref()),
        size: 32,
        class_list: "userAvatar",
        ..StyledAvatar::default()
    }
    .render();

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
                    styled_tip('m', model, EnableCommentBuilder)
                ]
            ],
            comments
        ],
    ]
}

#[derive(Debug)]
pub struct EnableCommentBuilder;

impl BuildMsg for EnableCommentBuilder {
    fn build(&self) -> Msg {
        Msg::ModalChanged(FieldChange::ToggleCommentForm(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            true,
        ))
    }
}

#[inline(always)]
fn right_modal_column(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        payload,
        status_state,
        reporter_state,
        assignees_state,
        priority_state,
        ..
    } = modal;

    let status_field = status_select(model, payload, status_state);

    let assignees_field = assignees_select(model, payload, assignees_state);

    let reporter_field = reporters_select(model, payload, reporter_state);

    let priority_field = priorities_select(payload, priority_state);

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
        let tracking_field = StyledField {
            label: "TIME TRACKING",
            tip: Some(""),
            input: tracking,
            ..Default::default()
        }
        .render();
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

#[inline(always)]
fn priorities_select(
    payload: &UpdateIssuePayload,
    priority_state: &StyledSelectState,
) -> Node<Msg> {
    let priority = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Priority)),
        name: "priority",
        variant: SelectVariant::Empty,
        opened: priority_state.opened,
        text_filter: priority_state.text_filter.as_str(),
        options: Some(
            IssuePriority::default()
                .into_iter()
                .map(priority_select_option),
        ),
        selected: vec![priority_select_option(payload.priority)],
        ..Default::default()
    }
    .render();
    StyledField {
        label: "Priority",
        input: priority,
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn priority_select_option<'l>(ip: IssuePriority) -> StyledSelectOption<'l> {
    StyledSelectOption {
        icon: Some(
            StyledIcon {
                icon: ip.into(),
                class_list: ip.to_str(),
                ..Default::default()
            }
            .render(),
        ),
        text: Some(ip.to_str()),
        class_list: ip.to_str(),
        value: ip.into(),
        name: Some("priority"),
        ..Default::default()
    }
}

#[inline(always)]
fn status_select(
    model: &Model,
    payload: &UpdateIssuePayload,
    status_state: &StyledSelectState,
) -> Node<Msg> {
    let status = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::IssueStatusId)),
        name: "status",
        opened: status_state.opened,
        variant: SelectVariant::Normal,
        text_filter: status_state.text_filter.as_str(),
        options: Some(model.issue_statuses.iter().map(issue_status_select_option)),
        selected: model
            .issue_statuses
            .iter()
            .filter(|is| is.id == payload.issue_status_id)
            .map(issue_status_select_option)
            .collect(),

        valid: true,
        ..Default::default()
    }
    .render();
    StyledField {
        label: "Status",
        input: status,
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn issue_status_select_option<'l>(is: &'l IssueStatus) -> StyledSelectOption<'l> {
    StyledSelectOption {
        value: is.id as u32,
        class_list: is.name.as_str(),
        text: Some(is.name.as_str()),
        name: Some("status"),
        ..Default::default()
    }
}

#[inline(always)]
fn reporters_select(
    model: &Model,
    payload: &UpdateIssuePayload,
    reporter_state: &StyledSelectState,
) -> Node<Msg> {
    let reporter = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Reporter)),
        name: "reporter",
        opened: reporter_state.opened,
        variant: SelectVariant::Empty,
        text_filter: reporter_state.text_filter.as_str(),
        options: Some(model.users.iter().map(reporter_select_option)),
        selected: model
            .users
            .iter()
            .filter(|user| payload.reporter_id == user.id)
            .map(reporter_select_option)
            .collect(),
        ..Default::default()
    }
    .render();
    StyledField {
        label: "Reporter",
        input: reporter,
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn reporter_select_option<'l>(user: &'l User) -> StyledSelectOption<'l> {
    StyledSelectOption {
        value: user.id as u32,
        icon: Some(
            StyledAvatar {
                size: 20,
                name: &user.name,
                avatar_url: user.avatar_url.as_deref(),
                ..StyledAvatar::default()
            }
            .render(),
        ),
        text: Some(user.name.as_str()),
        name: Some("reporter"),
        ..Default::default()
    }
}

#[inline(always)]
fn assignees_select(
    model: &Model,
    payload: &UpdateIssuePayload,
    assignees_state: &StyledSelectState,
) -> Node<Msg> {
    let assignees = StyledSelect {
        id: FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
        name: "assignees",
        variant: SelectVariant::Empty,
        is_multi: true,
        opened: assignees_state.opened,
        text_filter: assignees_state.text_filter.as_str(),
        options: Some(model.users.iter().map(assignee_select_option)),
        selected: model
            .users
            .iter()
            .filter(|user| payload.user_ids.contains(&user.id))
            .map(assignee_select_option)
            .collect(),
        ..Default::default()
    }
    .render();
    StyledField {
        input: assignees,
        label: "Assignees",
        ..Default::default()
    }
    .render()
}

#[inline(always)]
fn assignee_select_option<'l>(user: &'l User) -> StyledSelectOption<'l> {
    StyledSelectOption {
        value: user.id as u32,
        icon: Some(
            StyledAvatar {
                size: 20,
                name: &user.name,
                avatar_url: user.avatar_url.as_deref(),
                ..StyledAvatar::default()
            }
            .render(),
        ),
        text: Some(user.name.as_str()),
        name: Some("assignees"),
        ..Default::default()
    }
}
