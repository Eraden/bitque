use {
    crate::{
        components::styled_textarea::StyledTextarea, shared::ToNode, FieldChange, FieldId, Msg,
    },
    seed::{prelude::*, *},
};

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
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
    pub fn new<S: Into<String>>(mode: Mode, text: S) -> Self {
        Self {
            mode,
            initial_text: text.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyledEditor {
    id: FieldId,
    initial_text: String,
    text: String,
    html: String,
    mode: Mode,
    update_event: Ev,
}

impl StyledEditor {
    #[inline]
    pub fn build(id: FieldId) -> StyledEditorBuilder {
        StyledEditorBuilder {
            id,
            initial_text: "".to_string(),
            text: "".to_string(),
            html: "".to_string(),
            mode: Mode::View,
            update_event: None,
        }
    }
}

#[derive(Debug)]
pub struct StyledEditorBuilder {
    id: FieldId,
    initial_text: String,
    text: String,
    html: String,
    mode: Mode,
    update_event: Option<Ev>,
}

impl StyledEditorBuilder {
    #[inline]
    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.text = text.into();
        self
    }

    #[inline]
    pub fn initial_text<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.initial_text = text.into();
        self
    }

    #[inline]
    pub fn html<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.html = text.into();
        self
    }

    #[inline]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    #[inline]
    pub fn build(self) -> StyledEditor {
        StyledEditor {
            id: self.id,
            initial_text: self.initial_text,
            text: self.text,
            html: self.html,
            mode: self.mode,
            update_event: self.update_event.unwrap_or(Ev::KeyUp),
        }
    }

    #[inline]
    pub fn update_on(mut self, ev: Ev) -> Self {
        self.update_event = Some(ev);
        self
    }
}

impl ToNode for StyledEditor {
    #[inline]
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[inline]
pub fn render(values: StyledEditor) -> Node<Msg> {
    let StyledEditor {
        id,
        initial_text,
        text: _,
        html,
        mode,
        update_event,
    } = values;

    let on_editor_clicked = click_handler(id.clone(), Mode::Editor);
    let on_view_clicked = click_handler(id.clone(), Mode::View);

    let editor_id = format!("editor-{}", id);
    let view_id = format!("view-{}", id);
    let name = format!("styled-editor-{}", id);

    let text_area = StyledTextarea::build(id)
        .height(40)
        .update_on(update_event)
        // .disable_auto_resize()
        .value(initial_text.as_str())
        .build()
        .into_node();

    let (editor_radio_node, view_radio_node, parsed_node) = match mode {
        Mode::Editor => (
            seed::input![
                id![editor_id.as_str()],
                attrs![At::Type => "radio"; At::Name => name.as_str(); At::Class => "editorRadio"; At::Checked => true],
            ],
            seed::input![
                id![view_id.as_str()],
                attrs![ At::Type => "radio"; At::Name => name.as_str(); At::Class => "viewRadio";],
            ],
            vec![],
        ),
        Mode::View => (
            seed::input![
                id![editor_id.as_str()],
                C!["editorRadio"],
                attrs![At::Type => "radio"; At::Name => name.as_str();],
            ],
            seed::input![
                id![view_id.as_str()],
                C!["viewRadio"],
                attrs![ At::Type => "radio"; At::Name => name.as_str(); At::Checked => true],
            ],
            Node::from_html(None, html.as_str()),
        ),
    };

    div![
        C!["styledEditor"],
        label![
            if mode == Mode::View {
                C!["navbar viewTab activeTab"]
            } else {
                C!["navbar viewTab"]
            },
            attrs![At::For => view_id.as_str()],
            "View",
            on_view_clicked
        ],
        label![
            if mode == Mode::Editor {
                C!["navbar editorTab activeTab"]
            } else {
                C!["navbar editorTab"]
            },
            attrs![At::For => editor_id.as_str()],
            "Editor",
            on_editor_clicked
        ],
        editor_radio_node,
        text_area,
        view_radio_node,
        div![C!["view"], parsed_node],
    ]
}

#[inline]
fn click_handler(field_id: FieldId, new_mode: Mode) -> EventHandler<Msg> {
    mouse_ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        Msg::ModalChanged(FieldChange::TabChanged(field_id, new_mode))
    })
}
