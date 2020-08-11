use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use jirs_data::Epic;

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

#[derive(Serialize, Deserialize)]
pub struct LoadEpics {
    pub project_id: i32,
}

impl Message for LoadEpics {
    type Result = Result<Vec<Epic>, ServiceErrors>;
}

impl Handler<LoadEpics> for DbExecutor {
    type Result = Result<Vec<Epic>, ServiceErrors>;

    fn handle(&mut self, msg: LoadEpics, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let epics_query = epics.distinct_on(id).filter(project_id.eq(msg.project_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&epics_query));
        epics_query
            .load(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("epics".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateEpic {
    pub user_id: i32,
    pub project_id: i32,
    pub name: String,
}

impl Message for CreateEpic {
    type Result = Result<Epic, ServiceErrors>;
}

impl Handler<CreateEpic> for DbExecutor {
    type Result = Result<Epic, ServiceErrors>;

    fn handle(&mut self, msg: CreateEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let epic_query = diesel::insert_into(epics).values((
            name.eq(msg.name.as_str()),
            user_id.eq(msg.user_id),
            project_id.eq(msg.project_id),
        ));
        debug!("{}", diesel::debug_query::<Pg, _>(&epic_query));
        epic_query
            .get_result::<Epic>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("epics".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEpic {
    pub epic_id: i32,
    pub project_id: i32,
    pub name: String,
}

impl Message for UpdateEpic {
    type Result = Result<Epic, ServiceErrors>;
}

impl Handler<UpdateEpic> for DbExecutor {
    type Result = Result<Epic, ServiceErrors>;

    fn handle(&mut self, msg: UpdateEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let query = diesel::update(
            epics
                .filter(project_id.eq(msg.project_id))
                .find(msg.epic_id),
        )
        .set(name.eq(msg.name));
        info!("{}", diesel::debug_query::<Pg, _>(&query));
        let row: Epic = query
            .get_result::<Epic>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("epics".to_string()))?;
        Ok(row)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteEpic {
    pub epic_id: i32,
    pub user_id: i32,
}

impl Message for DeleteEpic {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<DeleteEpic> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: DeleteEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let comment_query = diesel::delete(epics.filter(user_id.eq(msg.user_id)).find(msg.epic_id));
        debug!("{}", diesel::debug_query::<Pg, _>(&comment_query));
        comment_query
            .execute(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("epics".to_string()))?;
        Ok(())
    }
}
