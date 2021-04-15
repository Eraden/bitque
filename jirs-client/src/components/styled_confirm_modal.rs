use {
    crate::{
        components::{styled_button::StyledButton, styled_modal::StyledModal},
        shared::ToNode,
        Msg,
    },
    seed::{prelude::*, EventHandler, *},
};

use crate::components::styled_button::ButtonVariant;

const TITLE: &str = "Warning";
const MESSAGE: &str = "Are you sure you want to continue with this action?";
const CONFIRM_TEXT: &str = "Confirm";
const CANCEL_TEXT: &str = "Cancel";

#[derive(Debug)]
pub struct StyledConfirmModal<'l> {
    pub title: &'l str,
    pub message: &'l str,
    pub confirm_text: &'l str,
    pub cancel_text: &'l str,
    pub on_confirm: Option<EventHandler<Msg>>,
}

impl<'l> Default for StyledConfirmModal<'l> {
    fn default() -> Self {
        Self {
            title: TITLE,
            message: MESSAGE,
            confirm_text: CONFIRM_TEXT,
            cancel_text: CANCEL_TEXT,
            on_confirm: None,
        }
    }
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
            title: self.title.unwrap_or(TITLE),
            message: self.message.unwrap_or(MESSAGE),
            confirm_text: self.confirm_text.unwrap_or(CONFIRM_TEXT),
            cancel_text: self.cancel_text.unwrap_or(CANCEL_TEXT),
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

    let title = if title.is_empty() { TITLE } else { title };
    let message = if message.is_empty() { MESSAGE } else { message };
    let confirm_text = if confirm_text.is_empty() {
        CONFIRM_TEXT
    } else {
        confirm_text
    };
    let cancel_text = if cancel_text.is_empty() {
        CANCEL_TEXT
    } else {
        cancel_text
    };

    let message_node = match message {
        _ if message.is_empty() => empty![],
        _ => p![attrs![At::Class => "message"], message],
    };

    let confirm_button = StyledButton {
        text: Some(confirm_text),
        on_click: on_confirm,
        ..Default::default()
    }
    .into_node();
    let cancel_button = StyledButton {
        text: Some(cancel_text),
        variant: ButtonVariant::Secondary,
        on_click: Some(mouse_ev(Ev::Click, |_| Msg::ModalDropped)),
        ..Default::default()
    }
    .into_node();

    StyledModal {
        width: Some(600),
        children: vec![
            div![C!["title"], title],
            message_node,
            div![C!["actions"], confirm_button, cancel_button],
        ],
        class_list: "confirmModal",
        ..Default::default()
    }
    .into_node()
}
