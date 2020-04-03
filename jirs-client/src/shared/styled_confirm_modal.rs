use seed::EventHandler;
use seed::{prelude::*, *};

use crate::shared::styled_button::{StyledButton, Variant as ButtonVariant};
use crate::shared::styled_modal::StyledModal;
use crate::shared::ToNode;
use crate::Msg;

const TITLE: &str = "Warning";
const MESSAGE: &str = "Are you sure you want to continue with this action?";
const CONFIRM_TEXT: &str = "Confirm";
const CANCEL_TEXT: &str = "Cancel";

#[derive(Debug)]
pub enum Variant {
    Primary,
}

#[derive(Debug)]
pub struct StyledConfirmModal {
    pub variant: Variant,
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub on_confirm: Option<EventHandler<Msg>>,
}

impl StyledConfirmModal {
    pub fn build() -> StyledConfirmModalBuilder {
        StyledConfirmModalBuilder::default()
    }
}

impl ToNode for StyledConfirmModal {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Default)]
pub struct StyledConfirmModalBuilder {
    variant: Option<Variant>,
    title: Option<String>,
    message: Option<String>,
    confirm_text: Option<String>,
    cancel_text: Option<String>,
    on_confirm: Option<Option<EventHandler<Msg>>>,
}

impl StyledConfirmModalBuilder {
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn message<S>(mut self, message: S) -> Self
    where
        S: Into<String>,
    {
        self.message = Some(message.into());
        self
    }

    pub fn confirm_text<S>(mut self, confirm_text: S) -> Self
    where
        S: Into<String>,
    {
        self.confirm_text = Some(confirm_text.into());
        self
    }

    pub fn cancel_text<S>(mut self, cancel_text: S) -> Self
    where
        S: Into<String>,
    {
        self.cancel_text = Some(cancel_text.into());
        self
    }

    pub fn on_confirm(mut self, on_confirm: EventHandler<Msg>) -> Self {
        self.on_confirm = Some(Some(on_confirm));
        self
    }

    pub fn build(self) -> StyledConfirmModal {
        StyledConfirmModal {
            variant: self.variant.unwrap_or_else(|| Variant::Primary),
            title: self.title.unwrap_or_else(|| TITLE.to_string()),
            message: self.message.unwrap_or_else(|| MESSAGE.to_string()),
            confirm_text: self
                .confirm_text
                .unwrap_or_else(|| CONFIRM_TEXT.to_string()),
            cancel_text: self.cancel_text.unwrap_or_else(|| CANCEL_TEXT.to_string()),
            on_confirm: self.on_confirm.unwrap_or_default(),
        }
    }
}

pub fn render(values: StyledConfirmModal) -> Node<Msg> {
    let StyledConfirmModal {
        variant,
        title,
        message,
        confirm_text,
        cancel_text,
        on_confirm,
    } = values;

    let message_node = match message {
        _ if message.is_empty() => empty![],
        _ => p![attrs![At::Class => "message"], message],
    };

    let confirm_button = match on_confirm {
        Some(handler) => StyledButton::build()
            .text(confirm_text)
            .on_click(handler)
            .build()
            .into_node(),
        _ => StyledButton::build().text(confirm_text).build().into_node(),
    };
    let cancel_button = StyledButton::build()
        .text(cancel_text)
        .variant(ButtonVariant::Secondary)
        .on_click(mouse_ev(Ev::Click, |_| Msg::ModalDropped))
        .build()
        .into_node();

    StyledModal::build()
        .width(600)
        .children(vec![
            div![attrs![At::Class => "title"], title],
            message_node,
            div![
                attrs![At::Class => "actions"],
                confirm_button,
                cancel_button
            ],
        ])
        .add_class("confirmModal".to_string())
        .build()
        .into_node()
}
