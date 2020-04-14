use actix::Addr;
use actix_web::web::Data;

use jirs_data::{UpdateProjectPayload, WsMsg};

use crate::db::projects::LoadCurrentProject;
use crate::db::DbExecutor;
use crate::ws::{current_user, WsResult};

pub async fn current_project(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
) -> WsResult {
    let project_id = current_user(user).map(|u| u.project_id)?;

    let m = match db.send(LoadCurrentProject { project_id }).await {
        Ok(Ok(project)) => Some(WsMsg::ProjectLoaded(project.into())),
        Ok(Err(e)) => {
            error!("{:?}", e);
            None
        }
        Err(e) => {
            error!("{:?}", e);
            None
        }
    };
    Ok(m)
}

pub async fn update_project(
    db: &Data<Addr<DbExecutor>>,
    user: &Option<jirs_data::User>,
    payload: UpdateProjectPayload,
) -> WsResult {
    let project_id = current_user(user).map(|u| u.project_id)?;
    let project = match db
        .send(crate::db::projects::UpdateProject {
            project_id,
            name: payload.name,
            url: payload.url,
            description: payload.description,
            category: payload.category,
        })
        .await
    {
        Ok(Ok(project)) => project,
        _ => return Ok(None),
    };
    Ok(Some(WsMsg::ProjectLoaded(project.into())))
}
