use seed::EventHandler;
use seed::{prelude::*, *};

use crate::shared::styled_button::StyledButton;
use crate::shared::styled_modal::StyledModal;
use crate::shared::ToNode;
use crate::Msg;

const TITLE: &str = "Warning";
const MESSAGE: &str = "Are you sure you want to continue with this action?";
const CONFIRM_TEXT: &str = "Confirm";
const CANCEL_TEXT: &str = "Cancel";

#[derive(Debug)]
pub struct StyledConfirmModal<'l> {
    title: &'l str,
    message: &'l str,
    confirm_text: &'l str,
    cancel_text: &'l str,
    on_confirm: Option<EventHandler<Msg>>,
}

impl<'l> StyledConfirmModal<'l> {
    pub fn build() -> StyledConfirmModalBuilder<'l> {
        StyledConfirmModalBuilder::default()
    }
}

impl<'l> ToNode for StyledConfirmModal<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Default)]
pub struct StyledConfirmModalBuilder<'l> {
    title: Option<&'l str>,
    message: Option<&'l str>,
    confirm_text: Option<&'l str>,
    cancel_text: Option<&'l str>,
    on_confirm: Option<EventHandler<Msg>>,
}

impl<'l> StyledConfirmModalBuilder<'l> {
    pub fn title(mut self, title: &'l str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn message(mut self, message: &'l str) -> Self {
        self.message = Some(message);
        self
    }

    pub fn confirm_text(mut self, confirm_text: &'l str) -> Self {
        self.confirm_text = Some(confirm_text);
        self
    }

    pub fn cancel_text(mut self, cancel_text: &'l str) -> Self {
        self.cancel_text = Some(cancel_text);
        self
    }

    pub fn on_confirm(mut self, on_confirm: EventHandler<Msg>) -> Self {
        self.on_confirm = Some(on_confirm);
        self
    }

    pub fn build(self) -> StyledConfirmModal<'l> {
        StyledConfirmModal {
            title: self.title.unwrap_or_else(|| TITLE),
            message: self.message.unwrap_or_else(|| MESSAGE),
            confirm_text: self.confirm_text.unwrap_or_else(|| CONFIRM_TEXT),
            cancel_text: self.cancel_text.unwrap_or_else(|| CANCEL_TEXT),
            on_confirm: self.on_confirm,
        }
    }
}

pub fn render(values: StyledConfirmModal) -> Node<Msg> {
    let StyledConfirmModal {
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
        .secondary()
        .on_click(mouse_ev(Ev::Click, |_| Msg::ModalDropped))
        .build()
        .into_node();

    StyledModal::build()
        .width(600)
        .child(div![C!["title"], title])
        .child(message_node)
        .child(div![C!["actions"], confirm_button, cancel_button])
        .add_class("confirmModal")
        .build()
        .into_node()
}
