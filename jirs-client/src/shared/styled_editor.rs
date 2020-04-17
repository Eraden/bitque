use seed::{prelude::*, *};

use crate::shared::styled_textarea::StyledTextarea;
use crate::shared::ToNode;
use crate::{FieldChange, FieldId, Msg};

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub enum Mode {
    Editor,
    View,
}

#[derive(Debug, Clone)]
pub struct StyledEditorState {
    pub mode: Mode,
}

#[derive(Debug, Clone)]
pub struct StyledEditor {
    id: FieldId,
    text: String,
    mode: Mode,
    update_event: Ev,
}

impl StyledEditor {
    pub fn build(id: FieldId) -> StyledEditorBuilder {
        StyledEditorBuilder {
            id,
            text: String::new(),
            mode: Mode::View,
            update_event: None,
        }
    }
}

#[derive(Debug)]
pub struct StyledEditorBuilder {
    id: FieldId,
    text: String,
    mode: Mode,
    update_event: Option<Ev>,
}

impl StyledEditorBuilder {
    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.text = text.into();
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn build(self) -> StyledEditor {
        StyledEditor {
            id: self.id,
            text: self.text,
            mode: self.mode,
            update_event: self.update_event.unwrap_or_else(|| Ev::KeyUp),
        }
    }

    pub fn update_on(mut self, ev: Ev) -> Self {
        self.update_event = Some(ev);
        self
    }
}

impl ToNode for StyledEditor {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledEditor) -> Node<Msg> {
    let StyledEditor {
        id,
        text,
        mode,
        update_event,
    } = values;

    let field_id = id.clone();
    let on_editor_clicked = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::TabChanged(field_id, Mode::Editor))
    });

    let field_id = id.clone();
    let on_view_clicked = mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::TabChanged(field_id, Mode::View))
    });

    let editor_id = format!("editor-{}", id);
    let view_id = format!("view-{}", id);
    let name = format!("styled-editor-{}", id);

    let text_area = StyledTextarea::build(id)
        .height(40)
        .update_on(update_event)
        .value(text.as_str())
        .build()
        .into_node();

    let parsed = comrak::markdown_to_html(
        text.as_str(),
        &comrak::ComrakOptions {
            hardbreaks: false,
            smart: true,
            github_pre_lang: true,
            width: 0,
            default_info_string: None,
            unsafe_: false,
            ext_strikethrough: true,
            ext_tagfilter: true,
            ext_table: true,
            ext_autolink: true,
            ext_tasklist: true,
            ext_superscript: true,
            ext_header_ids: None,
            ext_footnotes: true,
            ext_description_lists: true,
        },
    );
    let parsed_node = Node::from_html(parsed.as_str());

    let (editor_radio_node, view_radio_node) = match mode {
        Mode::Editor => (
            seed::input![
                id![editor_id.as_str()],
                attrs![At::Type => "radio"; At::Name => name.as_str(); At::Class => "editorRadio"; At::Checked => true],
            ],
            seed::input![
                id![view_id.as_str()],
                attrs![ At::Type => "radio"; At::Name => name.as_str(); At::Class => "viewRadio";],
            ],
        ),
        Mode::View => (
            seed::input![
                id![editor_id.as_str()],
                class!["editorRadio"],
                attrs![At::Type => "radio"; At::Name => name.as_str();],
            ],
            seed::input![
                id![view_id.as_str()],
                class!["viewRadio"],
                attrs![ At::Type => "radio"; At::Name => name.as_str(); At::Checked => true],
            ],
        ),
    };

    div![
        attrs![At::Class => "styledEditor"],
        label![
            if mode == Mode::Editor {
                class!["navbar editorTab activeTab"]
            } else {
                class!["navbar editorTab"]
            },
            attrs![At::For => editor_id.as_str()],
            "Editor",
            on_editor_clicked
        ],
        label![
            if mode == Mode::View {
                class!["navbar viewTab activeTab"]
            } else {
                class!["navbar viewTab"]
            },
            attrs![At::For => view_id.as_str()],
            "View",
            on_view_clicked
        ],
        editor_radio_node,
        text_area,
        view_radio_node,
        div![attrs![At::Class => "view"], parsed_node],
    ]
}
