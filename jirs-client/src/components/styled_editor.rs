use seed::prelude::*;
use seed::*;

use crate::components::styled_textarea::StyledTextarea;
use crate::{FieldChange, FieldId, Msg};

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
#[repr(C)]
pub enum Mode {
    Editor,
    View,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct StyledEditorState {
    pub mode: Mode,
    pub initial_text: String,
}

impl StyledEditorState {
    #[inline(always)]
    pub fn new<S: Into<String>>(mode: Mode, text: S) -> Self {
        Self {
            mode,
            initial_text: text.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyledEditor<'l> {
    pub id: Option<FieldId>,
    pub initial_text: &'l str,
    pub text: &'l str,
    pub html: &'l str,
    pub mode: Mode,
    pub update_event: Ev,
}

impl<'l> Default for StyledEditor<'l> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            id: None,
            initial_text: "",
            text: "",
            html: "",
            mode: Mode::Editor,
            update_event: Ev::Cached,
        }
    }
}

impl<'l> StyledEditor<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledEditor {
            id,
            initial_text,
            text: _,
            html,
            mode,
            update_event,
        } = self;

        let id = id.expect("Styled Editor requires ID");
        let on_editor_clicked = click_handler(id.clone(), Mode::Editor);
        let on_view_clicked = click_handler(id.clone(), Mode::View);

        let editor_id = format!("editor-{}", id);
        let view_id = format!("view-{}", id);
        let name = format!("styled-editor-{}", id);

        let text_area = StyledTextarea {
            id: Some(id),
            height: 40,
            value: initial_text,
            update_event,
            ..Default::default()
        }
        .render();

        div![
            C!["styledEditor"],
            label![
                C!["navbar viewTab", IF![mode == Mode::View => "activeTab"]],
                attrs![At::For => view_id.as_str()],
                "View",
                on_view_clicked
            ],
            label![
                C!["navbar editorTab", IF![mode == Mode::Editor => "activeTab"]],
                attrs![At::For => editor_id.as_str()],
                "Editor",
                on_editor_clicked
            ],
            seed::input![
                id![editor_id.as_str()],
                C!["editorRadio"],
                attrs![At::Type => "radio"; At::Name => name.as_str(); At::Checked => true],
            ],
            text_area,
            seed::input![
                id![view_id.as_str()],
                C!["viewRadio"],
                attrs![ At::Type => "radio"; At::Name => name.as_str();],
                IF![mode == Mode::View => attrs![At::Checked => true]]
            ],
            div![
                C!["view"],
                IF![mode == Mode::Editor => empty![]],
                IF![mode == Mode::View => raw![html]],
            ],
        ]
    }
}

#[inline(always)]
fn click_handler(field_id: FieldId, new_mode: Mode) -> EventHandler<Msg> {
    mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::TabChanged(field_id, new_mode))
    })
}
