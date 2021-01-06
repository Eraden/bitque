use {
    crate::{
        shared::{
            drag::DragState, styled_checkbox::StyledCheckboxState, styled_input::StyledInputState,
            styled_select::StyledSelectState,
        },
        FieldId,
    },
    jirs_data::{IssueStatusId, Project, ProjectFieldId, UpdateProjectPayload},
};

#[derive(Debug)]
pub struct ProjectSettingsPage {
    pub payload: UpdateProjectPayload,
    pub project_category_state: StyledSelectState,
    pub description_mode: crate::shared::styled_editor::Mode,
    pub time_tracking: StyledCheckboxState,
    pub column_drag: DragState,
    pub edit_column_id: Option<IssueStatusId>,
    pub creating_issue_status: bool,
    pub name: StyledInputState,
}

impl ProjectSettingsPage {
    pub fn new(project: &Project) -> Self {
        use crate::shared::styled_editor::Mode as EditorMode;
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
            description_mode: EditorMode::View,
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
        }
    }

    pub fn reset(&mut self) {
        self.edit_column_id = None;
        self.name.reset();
        self.creating_issue_status = false;
    }
}
