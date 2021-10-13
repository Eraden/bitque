use seed::prelude::*;
use seed::*;

use crate::components::styled_textarea::StyledTextarea;
use crate::{FieldChange, FieldId, Msg};

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
#[repr(C)]
pub enum MdEditorMode {
    Editor,
    View,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct StyledMdEditorState {
    pub id: FieldId,
    pub mode: MdEditorMode,
    pub initial_text: String,
    pub html: String,
}

impl StyledMdEditorState {
    #[inline(always)]
    pub fn new<Text: Into<String>, Html: Into<String>>(
        id: FieldId,
        mode: MdEditorMode,
        text: Text,
        html: Html,
    ) -> Self {
        Self {
            id,
            mode,
            initial_text: text.into(),
            html: html.into(),
        }
    }

    pub fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::ModalChanged(FieldChange::TabChanged(field_id, new_mode))
                if &self.id == field_id =>
            {
                self.mode = new_mode.clone();
            }
            _ => {}
        };
    }
}

#[derive(Debug, Clone)]
pub struct StyledMdEditor<'l> {
    pub id: Option<FieldId>,
    pub initial_text: &'l str,
    pub text: &'l str,
    pub html: &'l str,
    pub mode: MdEditorMode,
    pub update_event: Ev,
}

impl<'l> Default for StyledMdEditor<'l> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            id: None,
            initial_text: "",
            text: "",
            html: "",
            mode: MdEditorMode::Editor,
            update_event: Ev::Cached,
        }
    }
}

impl<'l> StyledMdEditor<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledMdEditor {
            id,
            initial_text,
            text,
            html,
            mode,
            update_event,
        } = self;

        let id = id.expect("Styled Editor requires ID");
        let on_editor_clicked =
            super::events::on_click_change_tab(id.clone(), MdEditorMode::Editor);
        let on_view_clicked = super::events::on_click_change_tab(id.clone(), MdEditorMode::View);

        let editor_id = format!("editor-{}", id);
        let view_id = format!("view-{}", id);
        let name = format!("styled-editor-{}", id);

        let text_area = StyledTextarea {
            id: Some(id),
            height: 40,
            value: if text.is_empty() { initial_text } else { text },
            update_event,
            ..Default::default()
        }
        .render();

        div![
            C!["styledEditor"],
            label![
                C![
                    "navbar viewTab",
                    IF![mode == MdEditorMode::View => "activeTab"]
                ],
                attrs![At::For => view_id.as_str()],
                "View",
                on_view_clicked
            ],
            label![
                C![
                    "navbar editorTab",
                    IF![mode == MdEditorMode::Editor => "activeTab"]
                ],
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
                IF![mode == MdEditorMode::View => attrs![At::Checked => true]]
            ],
            div![
                C!["view"],
                IF![mode == MdEditorMode::Editor => empty![]],
                IF![mode == MdEditorMode::View => raw![html]],
            ],
        ]
    }
}
