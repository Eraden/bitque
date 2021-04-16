use jirs_data::fields::*;
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_field::StyledField;
use crate::components::styled_form::StyledForm;
use crate::components::styled_input::StyledInput;
use crate::model::{Model, PageContent};
use crate::pages::invite_page::InvitePage;
use crate::shared::{outer_layout, ToNode};
use crate::validations::is_token;
use crate::{match_page, FieldId, InvitationPageChange, Msg, PageChanged};

pub fn view(model: &Model) -> Node<Msg> {
    let page = match_page!(model, Invite; Empty);

    let token_field = token_field(page);
    let submit_field = submit(page);
    let error = match page.error.as_ref() {
        Some(s) => div![C!["error"], s.as_str()],
        _ => empty![],
    };

    let form = StyledForm::build()
        .heading("Welcome in JIRS")
        .on_submit(ev(Ev::Submit, move |ev| {
            ev.prevent_default();
            Msg::PageChanged(PageChanged::Invitation(InvitationPageChange::SubmitForm))
        }))
        .add_field(token_field)
        .add_field(submit_field)
        .add_field(error)
        .build()
        .into_node();

    outer_layout(model, "invite", vec![form])
}

fn submit(_page: &InvitePage) -> Node<Msg> {
    let submit = StyledButton {
        text: Some("Accept"),
        variant: ButtonVariant::Primary,
        ..Default::default()
    }
    .into_node();
    StyledField {
        input: submit,
        ..Default::default()
    }
    .into_node()
}

fn token_field(page: &InvitePage) -> Node<Msg> {
    let input = StyledInput {
        valid: !page.token_touched || is_token(page.token.as_str()) && page.error.is_none(),
        id: Some(FieldId::Invite(InviteFieldId::Token)),
        value: page.token.as_str(),
        ..Default::default()
    }
    .into_node();

    StyledField {
        input,
        label: "Your invite token",
        ..Default::default()
    }
    .into_node()
}
