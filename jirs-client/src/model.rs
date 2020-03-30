use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::*;

type ProjectId = i32;

#[derive(Serialize, Deserialize)]
pub struct CreateCommentForm {
    fields: CreateCommentPayload,
}

#[derive(Serialize, Deserialize)]
pub struct CreateIssueForm {
    fields: CreateIssuePayload,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProjectForm {
    id: ProjectId,
    fields: UpdateProjectPayload,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    access_token: Option<Uuid>,
    user: Option<User>,
    project: Option<Project>,
    project_form: Option<UpdateProjectForm>,
    issue_form: Option<CreateIssueForm>,
    comment_form: Option<CreateCommentForm>,
    issues: Vec<Issue>,
    comments_by_project_id: HashMap<ProjectId, Vec<Comment>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            access_token: None,
            project: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            issues: vec![],
            comments_by_project_id: Default::default(),
        }
    }
}
