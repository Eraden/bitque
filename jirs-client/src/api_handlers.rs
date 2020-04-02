use seed::fetch::{FetchObject, ResponseWithDataResult};
use seed::*;

use jirs_data::{FullIssue, FullProject, Issue};

use crate::model::Model;

pub fn current_user_response(fetched: &FetchObject<String>, model: &mut Model) {
    if let FetchObject {
        result:
            Ok(ResponseWithDataResult {
                data: Ok(body),
                status,
                ..
            }),
        ..
    } = fetched
    {
        if status.is_error() {
            return;
        }
        match serde_json::from_str::<'_, jirs_data::User>(body.as_str()) {
            Ok(user) => {
                model.user = Some(user);
            }
            _ => (),
        }
    }
}

pub fn current_project_response(fetched: &FetchObject<String>, model: &mut Model) {
    if let FetchObject {
        result:
            Ok(ResponseWithDataResult {
                data: Ok(body),
                status,
                ..
            }),
        ..
    } = fetched
    {
        if status.is_error() {
            return;
        }
        match serde_json::from_str::<'_, FullProject>(body.as_str()) {
            Ok(project) => {
                model.project = Some(project);
            }
            _ => (),
        }
    }
}

pub fn update_issue_response(fetched: &FetchObject<String>, model: &mut Model) {
    if let FetchObject {
        result:
            Ok(ResponseWithDataResult {
                data: Ok(body),
                status,
                ..
            }),
        ..
    } = fetched
    {
        if status.is_error() {
            return;
        }
        match (
            serde_json::from_str::<'_, FullIssue>(body.as_str()),
            model.project.as_mut(),
        ) {
            (Ok(issue), Some(project)) => {
                let mut issues: Vec<Issue> = vec![];
                std::mem::swap(&mut project.issues, &mut issues);
                for i in issues.into_iter() {
                    if i.id != issue.id {
                        project.issues.push(i);
                    }
                }
                project.issues.push(issue.into());
            }
            (Err(error), _) => {
                error!(error);
            }
            _ => (),
        }
    }
}
