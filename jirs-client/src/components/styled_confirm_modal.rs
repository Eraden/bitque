use seed::prelude::*;
use seed::{EventHandler, *};

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_modal::StyledModal;
use crate::shared::ToNode;
use crate::Msg;

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

impl<'l> StyledConfirmModal<'l> {
    pub fn render(self) -> Node<Msg> {
        let StyledConfirmModal {
            title,
            message,
            confirm_text,
            cancel_text,
            on_confirm,
        } = self;

        let title = if title.is_empty() { TITLE } else { title };
        let message = if message.is_empty() { MESSAGE } else { message };

        let confirm_button = StyledButton {
            text: Some(match confirm_text {
                s if s.is_empty() => CONFIRM_TEXT,
                _ => confirm_text,
            }),
            on_click: on_confirm,
            ..Default::default()
        }
        .render();
        let cancel_button = StyledButton {
            text: Some(match cancel_text {
                s if s.is_empty() => CANCEL_TEXT,
                _ => cancel_text,
            }),
            variant: ButtonVariant::Secondary,
            on_click: Some(mouse_ev(Ev::Click, |_| Msg::ModalDropped)),
            ..Default::default()
        }
        .render();

        StyledModal {
            width: Some(600),
            children: vec![
                div![C!["title"], title],
                p![C!["message"], message],
                div![C!["actions"], confirm_button, cancel_button],
            ],
            class_list: "confirmModal",
            ..Default::default()
        }
        .into_node()
    }
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

impl<'l> ToNode for StyledConfirmModal<'l> {
    fn into_node(self) -> Node<Msg> {
        self.render()
    }
}
