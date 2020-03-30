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
