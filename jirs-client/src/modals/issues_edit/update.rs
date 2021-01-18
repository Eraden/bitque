use {
    crate::{
        components::styled_select::StyledSelectChanged,
        model::{IssueModal, Model},
        ws::send_ws_msg,
        EditIssueModalSection, FieldChange, FieldId, Msg, OperationKind, ResourceKind,
    },
    jirs_data::*,
    seed::prelude::*,
};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let modal = match &mut model.modals.edit_issue {
        Some(modal) => modal,
        _ => return,
    };
    modal.update_states(msg, orders);

    match msg {
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::SingleModified, Some(id)) => {
            let m = model.issues_by_id.get(id).cloned();
            if let Some(issue) = m {
                modal.description_state.initial_text = issue
                    .description_text
                    .as_deref()
                    .unwrap_or_default()
                    .to_string();
                modal.payload = issue.into();
            }
        }

        // type
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            modal.payload.issue_type = (*value).into();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Type,
                    PayloadVariant::IssueType(modal.payload.issue_type),
                ),
                model.ws.as_ref(),
                orders,
            );
        }

        // issue status id
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::IssueStatusId)),
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            modal.payload.issue_status_id = *value as IssueStatusId;
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::IssueStatusId,
                    PayloadVariant::I32(modal.payload.issue_status_id),
                ),
                model.ws.as_ref(),
                orders,
            );
        }

        // reporter id
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Reporter)),
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            modal.payload.reporter_id = *value as i32;
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Reporter,
                    PayloadVariant::I32(modal.payload.reporter_id),
                ),
                model.ws.as_ref(),
                orders,
            );
        }

        // assignees
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            modal.payload.user_ids.push(*value as i32);
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Assignees,
                    PayloadVariant::VecI32(modal.payload.user_ids.clone()),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
            StyledSelectChanged::RemoveMulti(value),
        ) => {
            let old = std::mem::replace(&mut modal.payload.user_ids, vec![]);
            let dropped = *value as i32;
            for id in old.into_iter() {
                if id != dropped {
                    modal.payload.user_ids.push(id);
                }
            }
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Assignees,
                    PayloadVariant::VecI32(modal.payload.user_ids.clone()),
                ),
                model.ws.as_ref(),
                orders,
            );
        }

        // priority
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Priority)),
            StyledSelectChanged::Changed(Some(value)),
        ) => {
            modal.payload.priority = (*value).into();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Priority,
                    PayloadVariant::IssuePriority(modal.payload.priority),
                ),
                model.ws.as_ref(),
                orders,
            );
        }

        // Title
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Title)),
            value,
        ) => {
            modal.payload.title = value.clone();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Title,
                    PayloadVariant::String(modal.payload.title.clone()),
                ),
                model.ws.as_ref(),
                orders,
            );
            orders.skip();
        }

        // Description
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Description)),
            value,
        ) => {
            modal.payload.description_text = Some(value.clone());
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Description,
                    PayloadVariant::String(
                        modal
                            .payload
                            .description_text
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                    ),
                ),
                model.ws.as_ref(),
                orders,
            );
            orders.skip();
        }
        // TimeSpent
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpent)),
            ..,
        ) => {
            modal.payload.time_spent = modal.time_spent.represent_f64_as_i32();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::TimeSpent,
                    PayloadVariant::OptionI32(modal.payload.time_spent),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpent)),
            StyledSelectChanged::Changed(..),
        ) => {
            modal.payload.time_spent = modal.time_spent_select.values.get(0).map(|n| *n as i32);
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::TimeSpent,
                    PayloadVariant::OptionI32(modal.payload.time_spent),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        // Time Remaining
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
            ..,
        ) => {
            modal.payload.time_remaining = modal.time_remaining.represent_f64_as_i32();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::TimeRemaining,
                    PayloadVariant::OptionI32(modal.payload.time_remaining),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
            StyledSelectChanged::Changed(..),
        ) => {
            modal.payload.time_remaining =
                modal.time_remaining_select.values.get(0).map(|n| *n as i32);
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::TimeRemaining,
                    PayloadVariant::OptionI32(modal.payload.time_remaining),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        // Estimate
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
            ..,
        ) => {
            modal.payload.estimate = modal.estimate.represent_f64_as_i32();
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Estimate,
                    PayloadVariant::OptionI32(modal.payload.estimate),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
            StyledSelectChanged::Changed(..),
        ) => {
            modal.payload.estimate = modal.estimate_select.values.get(0).map(|n| *n as i32);
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::Estimate,
                    PayloadVariant::OptionI32(modal.payload.estimate),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::StyledSelectChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::EpicName)),
            StyledSelectChanged::Changed(v),
        ) => {
            send_ws_msg(
                WsMsg::IssueUpdate(
                    modal.id,
                    IssueFieldId::EpicName,
                    PayloadVariant::OptionI32(v.map(|n| n as EpicId)),
                ),
                model.ws.as_ref(),
                orders,
            );
        }
        Msg::ModalChanged(FieldChange::TabChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Description)),
            mode,
        )) => {
            modal.description_state.mode = mode.clone();
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
        //
        // comments
        //
        Msg::StrInputChanged(
            FieldId::EditIssueModal(EditIssueModalSection::Comment(CommentFieldId::Body)),
            text,
        ) => {
            modal.comment_form.body = text.clone();
        }
        Msg::SaveComment => {
            let msg = match modal.comment_form.id {
                Some(id) => WsMsg::CommentUpdate(UpdateCommentPayload {
                    id,
                    body: modal.comment_form.body.clone(),
                }),
                _ => WsMsg::CommentCreate(CreateCommentPayload {
                    user_id: None,
                    body: modal.comment_form.body.clone(),
                    issue_id: modal.id,
                }),
            };
            send_ws_msg(msg, model.ws.as_ref(), orders);
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
            send_ws_msg(WsMsg::CommentDelete(*comment_id), model.ws.as_ref(), orders);
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
