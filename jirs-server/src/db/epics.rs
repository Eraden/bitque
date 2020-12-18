use actix::{Handler, Message};
use diesel::prelude::*;

use jirs_data::{msg::WsError, Epic};

use crate::{db::DbExecutor, db_pool, errors::ServiceError, q};

pub struct LoadEpics {
    pub project_id: i32,
}

impl Message for LoadEpics {
    type Result = Result<Vec<Epic>, ServiceError>;
}

impl Handler<LoadEpics> for DbExecutor {
    type Result = Result<Vec<Epic>, ServiceError>;

    fn handle(&mut self, msg: LoadEpics, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = db_pool!(self);

        q!(epics.distinct_on(id).filter(project_id.eq(msg.project_id)))
            .load(conn)
            .map_err(|e| {
                error!("{:?}", e);
                ServiceError::Error(WsError::FailedToLoadEpics)
            })
    }
}

pub struct CreateEpic {
    pub user_id: i32,
    pub project_id: i32,
    pub name: String,
}

impl Message for CreateEpic {
    type Result = Result<Epic, ServiceError>;
}

impl Handler<CreateEpic> for DbExecutor {
    type Result = Result<Epic, ServiceError>;

    fn handle(&mut self, msg: CreateEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = db_pool!(self);

        q!(diesel::insert_into(epics).values((
            name.eq(msg.name.as_str()),
            user_id.eq(msg.user_id),
            project_id.eq(msg.project_id),
        )))
        .get_result::<Epic>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::InvalidEpic)
        })
    }
}

pub struct UpdateEpic {
    pub epic_id: i32,
    pub project_id: i32,
    pub name: String,
}

impl Message for UpdateEpic {
    type Result = Result<Epic, ServiceError>;
}

impl Handler<UpdateEpic> for DbExecutor {
    type Result = Result<Epic, ServiceError>;

    fn handle(&mut self, msg: UpdateEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = db_pool!(self);

        q!(diesel::update(
            epics
                .filter(project_id.eq(msg.project_id))
                .find(msg.epic_id),
        )
        .set(name.eq(msg.name)))
        .get_result::<Epic>(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::FailedToUpdateEpic)
        })
    }
}

pub struct DeleteEpic {
    pub epic_id: i32,
    pub user_id: i32,
}

impl Message for DeleteEpic {
    type Result = Result<(), ServiceError>;
}

impl Handler<DeleteEpic> for DbExecutor {
    type Result = Result<(), ServiceError>;

    fn handle(&mut self, msg: DeleteEpic, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::epics::dsl::*;

        let conn = db_pool!(self);

        q!(diesel::delete(
            epics.filter(user_id.eq(msg.user_id)).find(msg.epic_id)
        ))
        .execute(conn)
        .map_err(|e| {
            error!("{:?}", e);
            ServiceError::Error(WsError::UnableToDeleteEpic)
        })?;

        Ok(())
    }
}
