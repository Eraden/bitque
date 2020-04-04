use seed::Method;

use jirs_data::UpdateIssuePayload;

use crate::shared::host_client;
use crate::Msg;

pub async fn fetch_current_project(host_url: String) -> Result<Msg, Msg> {
    match host_client(host_url, "/project") {
        Ok(client) => client.fetch_string(Msg::CurrentProjectResult).await,
        Err(e) => Err(Msg::InternalFailure(e)),
    }
}

pub async fn fetch_current_user(host_url: String) -> Result<Msg, Msg> {
    match host_client(host_url, "/currentUser") {
        Ok(client) => client.fetch_string(Msg::CurrentUserResult).await,
        Err(e) => Err(Msg::InternalFailure(e)),
    }
}

pub async fn update_issue(
    host_url: String,
    id: i32,
    payload: UpdateIssuePayload,
) -> Result<Msg, Msg> {
    match host_client(host_url, format!("/issues/{id}", id = id).as_str()) {
        Ok(client) => {
            client
                .method(Method::Put)
                .header("Content-Type", "application/json")
                .body_json(&payload)
                .fetch_string(Msg::IssueUpdateResult)
                .await
        }
        Err(e) => return Ok(Msg::InternalFailure(e)),
    }
}

pub async fn delete_issue(host_url: String, id: i32) -> Result<Msg, Msg> {
    match host_client(host_url, format!("/issues/{id}", id = id).as_str()) {
        Ok(client) => {
            client
                .method(Method::Delete)
                .header("Content-Type", "application/json")
                .fetch_string(Msg::IssueDeleteResult)
                .await
        }
        Err(e) => return Ok(Msg::InternalFailure(e)),
    }
}
