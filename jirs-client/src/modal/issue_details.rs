use seed::{prelude::*, *};

use jirs_data::*;

use crate::api::send_ws_msg;
use crate::model::{CommentForm, EditIssueModal, ModalType, Model};
use crate::shared::styled_avatar::StyledAvatar;
use crate::shared::styled_button::StyledButton;
use crate::shared::styled_editor::StyledEditor;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_icon::Icon;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::{StyledSelect, StyledSelectChange};
use crate::shared::styled_select_child::ToStyledSelectChild;
use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::tracking_widget::tracking_link;
use crate::shared::ToNode;
use crate::{EditIssueModalSection, FieldChange, FieldId, Msg};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let modal: &mut EditIssueModal = match model.modals.get_mut(0) {
        Some(ModalType::EditIssue(_issue_id, modal)) => modal,
        _ => return,
    };
    modal.top_type_state.update(msg, orders);
    modal.status_state.update(msg, orders);
    modal.reporter_state.update(msg, orders);
    modal.assignees_state.update(msg, orders);
    modal.priority_state.update(msg, orders);

    match msg {
        Msg::WsMsg(WsMsg::IssueUpdated(issue)) => {
            modal.payload = issue.clone().into();
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.issue_type = (*value).into();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Type,
                PayloadVariant::IssueType(modal.payload.issue_type.clone()),
            ));
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Status)),
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.status = (*value).into();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Status,
                PayloadVariant::IssueStatus(modal.payload.status.clone()),
            ));
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Reporter)),
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.reporter_id = *value as i32;
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Reporter,
                PayloadVariant::I32(modal.payload.reporter_id),
            ));
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.user_ids.push(*value as i32);
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Assignees,
                PayloadVariant::VecI32(modal.payload.user_ids.clone()),
            ));
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
            StyledSelectChange::RemoveMulti(value),
        ) => {
            let mut old = vec![];
            std::mem::swap(&mut old, &mut modal.payload.user_ids);
            let dropped = *value as i32;
            for id in old.into_iter() {
                if id != dropped {
                    modal.payload.user_ids.push(id);
                }
            }
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Assignees,
                PayloadVariant::VecI32(modal.payload.user_ids.clone()),
            ));
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Priority)),
            StyledSelectChange::Changed(value),
        ) => {
            modal.payload.priority = (*value).into();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Priority,
                PayloadVariant::IssuePriority(modal.payload.priority),
            ));
        }
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Title)),
            value,
        ) => {
            modal.payload.title = value.clone();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Title,
                PayloadVariant::String(modal.payload.title.clone()),
            ));
        }
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Description)),
            value,
        ) => {
            modal.payload.description = Some(value.clone());
            modal.payload.description_text = Some(value.clone());
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::Description,
                PayloadVariant::String(
                    modal
                        .payload
                        .description
                        .as_ref()
                        .cloned()
                        .unwrap_or_default(),
                ),
            ));
        }
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpend)),
            value,
        ) => {
            modal.payload.time_spent = value.parse::<i32>().ok();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::TimeSpend,
                PayloadVariant::OptionI32(modal.payload.time_spent.clone()),
            ));
        }
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
            value,
        ) => {
            modal.payload.time_remaining = value.parse::<i32>().ok();
            send_ws_msg(WsMsg::IssueUpdateRequest(
                modal.id,
                IssueFieldId::TimeRemaining,
                PayloadVariant::OptionI32(modal.payload.time_remaining.clone()),
            ));
        }
        Msg::ModalChanged(FieldChange::TabChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Description)),
            mode,
        )) => {
            modal.description_editor_mode = mode.clone();
        }
        Msg::ModalChanged(FieldChange::ToggleCommentForm(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            flag,
        )) => {
            modal.comment_form.creating = *flag;
            if !*flag {
                modal.comment_form.body.clear();
                modal.comment_form.id = None;
            }
        }
        // comments
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            text,
        ) => {
            modal.comment_form.body = text.clone();
        }
        Msg::InputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
            value,
        ) => match value.parse::<i32>() {
            Ok(n) if !value.is_empty() => {
                modal.payload.estimate = Some(n);
                send_ws_msg(WsMsg::IssueUpdateRequest(
                    modal.id,
                    IssueFieldId::TimeRemaining,
                    PayloadVariant::OptionI32(modal.payload.estimate.clone()),
                ));
            }
            _ if value.is_empty() => {
                modal.payload.estimate = None;
                send_ws_msg(WsMsg::IssueUpdateRequest(
                    modal.id,
                    IssueFieldId::TimeRemaining,
                    PayloadVariant::OptionI32(modal.payload.estimate.clone()),
                ));
            }
            _ => {}
        },
        Msg::SaveComment => {
            let msg = match modal.comment_form.id {
                Some(id) => WsMsg::UpdateComment(UpdateCommentPayload {
                    id,
                    body: modal.comment_form.body.clone(),
                }),
                _ => WsMsg::CreateComment(CreateCommentPayload {
                    user_id: None,
                    body: modal.comment_form.body.clone(),
                    issue_id: modal.id,
                }),
            };
            send_ws_msg(msg);
            orders
                .skip()
                .send_msg(Msg::ModalChanged(FieldChange::ToggleCommentForm(
                    FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                    false,
                )));
        }
        Msg::ModalChanged(FieldChange::EditComment(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            comment_id,
        )) => {
            let id = *comment_id;
            let body = model
                .comments
                .iter()
                .find(|c| c.id == id)
                .map(|c| c.body.clone())
                .unwrap_or_default();
            modal.comment_form.body = body;
            modal.comment_form.id = Some(id);
            modal.comment_form.creating = true;
        }
        Msg::DeleteComment(comment_id) => {
            send_ws_msg(WsMsg::CommentDeleteRequest(*comment_id));
            orders.skip().send_msg(Msg::ModalDropped);
        }

        // global
        Msg::GlobalKeyDown { key, .. } if key.as_str() == "m" && !modal.comment_form.creating => {
            orders
                .skip()
                .send_msg(Msg::ModalChanged(FieldChange::ToggleCommentForm(
                    FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                    true,
                )));
        }

        _ => (),
    }
}

pub fn view(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    div![
        attrs![At::Class => "issueDetails"],
        top_modal_row(model, modal),
        div![
            attrs![At::Class => "content"],
            left_modal_column(model, modal),
            right_modal_column(model, modal),
        ],
    ]
}

fn top_modal_row(_model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        id,
        payload,
        top_type_state,
        link_copied,
        ..
    } = modal;

    let issue_id = *id;

    let click_handler = mouse_ev(Ev::Click, move |_| {
        use wasm_bindgen::JsCast;

        let link = format!("http://localhost:7000/issues/{id}", id = issue_id);
        let el = match seed::html_document().create_element("textarea") {
            Ok(el) => el
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .clone(),
            _ => return Msg::NoOp,
        };
        seed::body().append_child(&el).unwrap();
        el.set_text_content(Some(link.as_str()));
        el.select();
        el.set_selection_range(0, 9999).unwrap();
        seed::html_document().exec_command("copy").unwrap();
        seed::body().remove_child(&el).unwrap();
        Msg::ModalChanged(FieldChange::LinkCopied(FieldId::CopyButtonLabel, true))
    });
    let close_handler = mouse_ev(Ev::Click, |_| Msg::ModalDropped);
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

    let issue_type_select = StyledSelect::build(FieldId::EditIssueModal(
        EditIssueModalSection::Issue(IssueFieldId::Type),
    ))
    .dropdown_width(150)
    .name("type")
    .text_filter(top_type_state.text_filter.as_str())
    .opened(top_type_state.opened)
    .valid(true)
    .options(
        IssueType::ordered()
            .into_iter()
            .map(|t| t.to_select_child().name("type"))
            .collect(),
    )
    .selected(vec![{
        let id = modal.id;
        let issue_type = &payload.issue_type;
        issue_type
            .to_select_child()
            .name("type")
            .text(format!("{} - {}", issue_type, id))
    }])
    .build()
    .into_node();

    div![
        attrs![At::Class => "topActions"],
        issue_type_select,
        div![
            attrs![At::Class => "topActionsRight"],
            copy_button,
            delete_button,
            close_button
        ],
    ]
}

fn left_modal_column(model: &Model, modal: &EditIssueModal) -> Node<Msg> {
    let EditIssueModal {
        payload,
        description_editor_mode,
        comment_form,
        ..
    } = modal;

    let title = StyledInput::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Title,
    )))
    .add_input_class("issueSummary")
    .add_wrapper_class("issueSummary")
    .add_wrapper_class("textarea")
    .value(payload.title.as_str())
    .valid(payload.title.len() >= 3)
    .build()
    .into_node();

    let description_text = payload.description.as_ref().cloned().unwrap_or_default();
    let description = StyledEditor::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Description,
    )))
    .text(description_text)
    .mode(description_editor_mode.clone())
    .update_on(Ev::Change)
    .build()
    .into_node();
    let description_field = StyledField::build().input(description).build().into_node();

    let user_avatar = StyledAvatar::build()
        .add_class("userAvatar")
        .size(32)
        .avatar_url(
            model
                .user
                .as_ref()
                .and_then(|u| u.avatar_url.clone())
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
        vec![div![class!["fakeTextArea"], "Add a comment...", handler]]
    };

    let comments: Vec<Node<Msg>> = model
        .comments
        .iter()
        .flat_map(|c| comment(model, modal, c))
        .collect();

    div![
        class!["left"],
        title,
        description_field,
        div![
            class!["comments"],
            div![class!["title"], "Comments"],
            div![
                class!["create"],
                user_avatar,
                div![
                    class!["right"],
                    create_comment,
                    div![
                        class!["proTip"],
                        strong![class!["strong"], "Pro tip: "],
                        "press ",
                        span![class!["tipLetter"], "M"],
                        " to comment"
                    ]
                ]
            ],
            comments
        ],
    ]
}

fn build_comment_form(form: &CommentForm) -> Vec<Node<Msg>> {
    use crate::shared::styled_button::Variant as ButtonVariant;

    let submit_comment_form = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::SaveComment
    });
    let close_comment_form = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::ToggleCommentForm(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            false,
        ))
    });

    let text_area = StyledTextarea::build(FieldId::EditIssueModal(EditIssueModalSection::Comment(
        CommentFieldId::Body,
    )))
    .value(form.body.as_str())
    .placeholder("Add a comment...")
    .build()
    .into_node();

    let submit = StyledButton::build()
        .variant(ButtonVariant::Primary)
        .on_click(submit_comment_form)
        .text("Save")
        .build()
        .into_node();
    let cancel = StyledButton::build()
        .variant(ButtonVariant::Empty)
        .on_click(close_comment_form)
        .text("Cancel")
        .build()
        .into_node();

    vec![text_area, div![class!["actions"], submit, cancel]]
}

fn comment(model: &Model, modal: &EditIssueModal, comment: &Comment) -> Option<Node<Msg>> {
    use crate::shared::styled_button::Variant as ButtonVariant;

    let show_form = modal.comment_form.creating && modal.comment_form.id == Some(comment.id);

    let user = model.users.iter().find(|u| u.id == comment.user_id)?;

    let avatar = StyledAvatar::build()
        .size(32)
        .avatar_url(user.avatar_url.as_ref().cloned()?)
        .add_class("userAvatar")
        .build()
        .into_node();

    let buttons = if model.user.as_ref().map(|u| u.id) == Some(comment.user_id) {
        let comment_id = comment.id;
        let delete_comment_handler = mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            Msg::ModalOpened(Box::new(ModalType::DeleteCommentConfirm(comment_id)))
        });
        let edit_button = StyledButton::build()
            .add_class("editButton")
            .on_click(mouse_ev(Ev::Click, move |_| {
                Msg::ModalChanged(FieldChange::EditComment(
                    FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
                    comment_id,
                ))
            }))
            .text("Edit")
            .variant(ButtonVariant::Empty)
            .build()
            .into_node();

        let cancel_button = StyledButton::build()
            .add_class("deleteButton")
            .on_click(delete_comment_handler)
            .text("Delete")
            .variant(ButtonVariant::Empty)
            .build()
            .into_node();

        vec![edit_button, cancel_button]
    } else {
        vec![]
    };

    let content = if show_form {
        div![class!["content"], build_comment_form(&modal.comment_form)]
    } else {
        div![
            class!["content"],
            div![class!["userName"], user.name.as_str()],
            p![class!["body"], comment.body],
            buttons,
        ]
    };

    let node = div![class!["styledComment"], avatar, content];
    Some(node)
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

    let status = StyledSelect::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Status,
    )))
    .name("status")
    .opened(status_state.opened)
    .normal()
    .text_filter(status_state.text_filter.as_str())
    .options(
        IssueStatus::ordered()
            .into_iter()
            .map(|opt| opt.to_select_child().name("status"))
            .collect(),
    )
    .selected(vec![payload.status.to_select_child().name("status")])
    .valid(true)
    .build()
    .into_node();
    let status_field = StyledField::build()
        .input(status)
        .label("Status")
        .build()
        .into_node();

    let assignees = StyledSelect::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Assignees,
    )))
    .name("assignees")
    .opened(assignees_state.opened)
    .empty()
    .multi()
    .text_filter(assignees_state.text_filter.as_str())
    .options(
        model
            .users
            .iter()
            .map(|user| user.to_select_child().name("assignees"))
            .collect(),
    )
    .selected(
        model
            .users
            .iter()
            .filter(|user| payload.user_ids.contains(&user.id))
            .map(|user| user.to_select_child().name("assignees"))
            .collect(),
    )
    .build()
    .into_node();
    let assignees_field = StyledField::build()
        .input(assignees)
        .label("Assignees")
        .build()
        .into_node();

    let reporter = StyledSelect::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Reporter,
    )))
    .name("reporter")
    .opened(reporter_state.opened)
    .empty()
    .text_filter(reporter_state.text_filter.as_str())
    .options(
        model
            .users
            .iter()
            .map(|user| user.to_select_child().name("reporter"))
            .collect(),
    )
    .selected(
        model
            .users
            .iter()
            .filter(|user| payload.reporter_id == user.id)
            .map(|user| user.to_select_child().name("reporter"))
            .collect(),
    )
    .build()
    .into_node();
    let reporter_field = StyledField::build()
        .input(reporter)
        .label("Reporter")
        .build()
        .into_node();

    let priority = StyledSelect::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Priority,
    )))
    .name("priority")
    .opened(priority_state.opened)
    .empty()
    .text_filter(priority_state.text_filter.as_str())
    .options(
        IssuePriority::ordered()
            .into_iter()
            .map(|p| p.to_select_child().name("priority"))
            .collect(),
    )
    .selected(vec![payload.priority.to_select_child().name("priority")])
    .build()
    .into_node();
    let priority_field = StyledField::build()
        .input(priority)
        .label("Priority")
        .build()
        .into_node();

    let estimate = StyledInput::build(FieldId::EditIssueModal(EditIssueModalSection::Issue(
        IssueFieldId::Estimate,
    )))
    .valid(true)
    .value(
        payload
            .estimate
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_default(),
    )
    .build()
    .into_node();
    let estimate_field = StyledField::build()
        .input(estimate)
        .label("Original Estimate (hours)")
        .build()
        .into_node();

    let tracking = tracking_link(model, modal);
    let tracking_field = StyledField::build()
        .label("TIME TRACKING")
        .tip("")
        .input(tracking)
        .build()
        .into_node();

    div![
        attrs![At::Class => "right"],
        status_field,
        assignees_field,
        reporter_field,
        priority_field,
        estimate_field,
        tracking_field,
    ]
}
