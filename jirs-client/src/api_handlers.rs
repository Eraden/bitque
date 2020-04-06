use seed::fetch::{FetchObject, ResponseWithDataResult};
use seed::*;

use jirs_data::{FullIssue, Issue};

use crate::model::Model;

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
        match serde_json::from_str::<'_, FullIssue>(body.as_str()) {
            Ok(issue) => {
                let mut issues: Vec<Issue> = vec![];
                std::mem::swap(&mut model.issues, &mut issues);
                for i in issues.into_iter() {
                    if i.id != issue.id {
                        model.issues.push(i);
                    }
                }
                model.issues.push(issue.into());
            }
            Err(error) => {
                error!(error);
            }
        }
    }
}
