use jirs_data::{IssueStatusId, Project, ProjectFieldId, TextEditorMode, UpdateProjectPayload};

use crate::components::styled_checkbox::StyledCheckboxState;
use crate::components::styled_editor::StyledEditorState;
use crate::components::styled_input::StyledInputState;
use crate::components::styled_select::StyledSelectState;
use crate::shared::drag::DragState;
use crate::FieldId;

#[derive(Debug)]
pub struct ProjectSettingsPage {
    pub payload: UpdateProjectPayload,
    pub project_category_state: StyledSelectState,
    pub time_tracking: StyledCheckboxState,
    pub column_drag: DragState,
    pub edit_column_id: Option<IssueStatusId>,
    pub creating_issue_status: bool,
    pub name: StyledInputState,
    pub description: StyledEditorState,
}

impl ProjectSettingsPage {
    pub fn new(mode: TextEditorMode, project: &Project) -> Self {
        let jirs_data::Project {
            id,
            name,
            url,
            description,
            category,
            time_tracking,
            ..
        } = project;
        Self {
            payload: UpdateProjectPayload {
                id: *id,
                name: Some(name.clone()),
                url: Some(url.clone()),
                description: Some(description.clone()),
                category: Some(*category),
                time_tracking: Some(*time_tracking),
            },
            project_category_state: StyledSelectState::new(
                FieldId::ProjectSettings(ProjectFieldId::Category),
                vec![(*category).into()],
            ),
            time_tracking: StyledCheckboxState::new(
                FieldId::ProjectSettings(ProjectFieldId::TimeTracking),
                (*time_tracking).into(),
            ),
            column_drag: Default::default(),
            edit_column_id: None,
            creating_issue_status: false,
            name: StyledInputState::new(
                FieldId::ProjectSettings(ProjectFieldId::IssueStatusName),
                "",
            ),
            description: StyledEditorState::new(
                FieldId::ProjectSettings(ProjectFieldId::DescriptionMode),
                mode,
                "",
                "",
            ),
        }
    }

    pub fn reset(&mut self) {
        self.edit_column_id = None;
        self.name.reset();
        self.creating_issue_status = false;
    }
}
