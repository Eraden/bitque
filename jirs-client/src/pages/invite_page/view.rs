use {
    crate::{
        components::{
            styled_button::StyledButton, styled_field::StyledField, styled_form::StyledForm,
            styled_input::StyledInput,
        },
        match_page,
        model::{Model, PageContent},
        pages::invite_page::InvitePage,
        shared::{outer_layout, ToNode},
        validations::is_token,
        FieldId, InvitationPageChange, Msg, PageChanged,
    },
    jirs_data::fields::*,
    seed::{prelude::*, *},
};

use crate::components::styled_button::ButtonVariant;

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
