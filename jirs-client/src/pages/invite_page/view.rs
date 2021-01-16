use {
    crate::{
        components::{
            styled_button::StyledButton, styled_field::StyledField, styled_form::StyledForm,
            styled_input::StyledInput,
        },
        model::{Model, PageContent},
        pages::invite_page::InvitePage,
        shared::{outer_layout, ToNode},
        validations::is_token,
        FieldId, InvitationPageChange, Msg, PageChanged,
    },
    jirs_data::fields::*,
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let page = match &model.page_content {
        PageContent::Invite(page) => page,
        _ => return empty![],
    };

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
    let submit = StyledButton::build()
        .text("Accept")
        .primary()
        .build()
        .into_node();
    StyledField::build().input(submit).build().into_node()
}

fn token_field(page: &InvitePage) -> Node<Msg> {
    let token = StyledInput::build()
        .valid(!page.token_touched || is_token(page.token.as_str()) && page.error.is_none())
        .value(page.token.as_str())
        .build(FieldId::Invite(InviteFieldId::Token))
        .into_node();

    StyledField::build()
        .input(token)
        .label("Your invite token")
        .build()
        .into_node()
}
