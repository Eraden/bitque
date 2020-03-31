use seed::fetch::{FetchObject, ResponseWithDataResult};
use seed::*;

use jirs_data::{FullProjectResponse, Issue};

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
        match serde_json::from_str::<'_, FullProjectResponse>(body.as_str()) {
            Ok(project_response) => {
                model.project = Some(project_response.project);
            }
            _ => (),
        }
    }
}

pub fn update_issue_response(fetched: &FetchObject<String>, model: &mut Model) {
    log!("update_issue_response");
    log!(fetched);
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
            serde_json::from_str::<'_, Issue>(body.as_str()),
            model.project.as_mut(),
        ) {
            (Ok(issue), Some(project)) => {
                let mut issues: Vec<Issue> = vec![];
                for i in project.issues.iter() {
                    if i.id != issue.id {
                        issues.push(i.clone());
                    }
                }
                issues.push(issue);
                project.issues = issues;
            }
            _ => (),
        }
    }
}
