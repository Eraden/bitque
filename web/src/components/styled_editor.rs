use jirs_data::TextEditorMode;
use seed::prelude::*;
use seed::*;

use crate::components::styled_checkbox::{ChildBuilder, StyledCheckbox, StyledCheckboxState};
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_md_editor::{MdEditorMode, StyledMdEditor, StyledMdEditorState};
use crate::components::styled_rte::{StyledRte, StyledRteState};
use crate::{FieldId, Msg};

#[derive(Debug)]
pub enum EditorMode {
    Md(StyledMdEditorState),
    Rte(StyledRteState),
}

#[derive(Debug)]
pub struct StyledEditorState {
    pub field_id: FieldId,
    pub state: EditorMode,
    pub current_mode: StyledCheckboxState,
    pub user_mode: TextEditorMode,
}

impl StyledEditorState {
    pub fn new(field_id: FieldId, user_mode: TextEditorMode, value: &str, html: &str) -> Self {
        Self {
            current_mode: StyledCheckboxState::new(
                field_id.clone(),
                match user_mode {
                    TextEditorMode::RteOnly => TextEditorMode::RteOnly,
                    TextEditorMode::MdOnly | TextEditorMode::Mixed => TextEditorMode::MdOnly,
                }
                .into(),
            ),
            state: build_state(user_mode, field_id.clone(), value, html),
            field_id,
            user_mode,
        }
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::U32InputChanged(field_id, mode)
                if &self.field_id == field_id && self.current_mode.value != *mode =>
            {
                let mode: TextEditorMode = (*mode).into();

                self.state = build_state(mode, self.field_id.clone(), "", "")
            }
            _ => {}
        };
        self.current_mode.update(msg);
        self.state.update(msg, orders);
    }

    pub fn set_content(&mut self, text: &str, html: &str) {
        match &mut self.state {
            EditorMode::Md(md) => {
                md.initial_text = text.to_string();
                md.html = html.to_string();
            }
            EditorMode::Rte(rte) => {
                rte.value = html.to_string();
            }
        }
    }
}

impl EditorMode {
    #[inline(always)]
    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        match self {
            EditorMode::Md(state) => state.update(msg),
            EditorMode::Rte(state) => state.update(msg, orders),
        };
    }
}

/// Build project description text area with styled field wrapper
#[inline(always)]
pub fn render_styled_editor(state: &StyledEditorState) -> Node<Msg> {
    let editor = match &state.state {
        EditorMode::Md(state) => render_md(state),
        EditorMode::Rte(state) => render_rte(state),
    };
    let switcher = render_mode_switcher(state);
    div![switcher, editor]
}

#[inline(always)]
pub fn render_mode_switcher(state: &StyledEditorState) -> Node<Msg> {
    match state.user_mode {
        TextEditorMode::Mixed => StyledCheckbox {
            options: Some(
                vec![TextEditorMode::MdOnly, TextEditorMode::RteOnly]
                    .into_iter()
                    .map(|tem| editor_mode_checkbox_option(tem, &state.current_mode)),
            ),
            class_list: "textEditorModeSwitcher",
        }
        .render(),
        _ => Node::Empty,
    }
}

#[inline(always)]
fn editor_mode_checkbox_option<'l>(
    tem: TextEditorMode,
    state: &StyledCheckboxState,
) -> ChildBuilder<'l> {
    let value: u32 = tem.into();
    ChildBuilder {
        field_id: state.field_id.clone(),
        selected: state.value == value,
        label: match tem {
            TextEditorMode::MdOnly => "Simple",
            TextEditorMode::RteOnly => "Rich Text Editor",
            TextEditorMode::Mixed => "Editor with possibility to switch between modes",
        },
        class_list: tem.to_str(),
        value,
        icon: match tem {
            TextEditorMode::MdOnly => StyledIcon::from(Icon::MdEditor).render(),
            TextEditorMode::RteOnly => StyledIcon::from(Icon::RteEditor).render(),
            TextEditorMode::Mixed => Node::Empty,
        },
        ..Default::default()
    }
}

/// Build project description text area with styled field wrapper
#[inline(always)]
fn render_md(state: &StyledMdEditorState) -> Node<Msg> {
    StyledMdEditor {
        id: Some(state.id.clone()),
        initial_text: state.initial_text.as_str(),
        text: state.initial_text.as_str(),
        html: state.html.as_str(),
        mode: state.mode.clone(),
        update_event: Ev::Change,
    }
    .render()
}

/// Build project description text area with styled field wrapper
#[inline(always)]
fn render_rte(state: &StyledRteState) -> Node<Msg> {
    let id = state.field_id.clone();
    StyledRte {
        field_id: id,
        table_tooltip: Some(&state.table_tooltip),
        code_tooltip: Some(&state.code_tooltip),
        identifier: Some(state.identifier),
    }
    .render()
}

fn build_state(
    user_mode: TextEditorMode,
    field_id: FieldId,
    value: &str,
    html: &str,
) -> EditorMode {
    match user_mode {
        TextEditorMode::RteOnly => build_state_rte(field_id),
        TextEditorMode::MdOnly | TextEditorMode::Mixed => build_state_md(field_id, value, html),
    }
}

#[inline(always)]
fn build_state_rte(field_id: FieldId) -> EditorMode {
    EditorMode::Rte(StyledRteState::new(field_id))
}

#[inline(always)]
fn build_state_md(field_id: FieldId, text: &str, html: &str) -> EditorMode {
    EditorMode::Md(StyledMdEditorState::new(
        field_id,
        MdEditorMode::View,
        text,
        html,
    ))
}
