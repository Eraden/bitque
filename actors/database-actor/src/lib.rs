#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
pub use errors::*;

pub mod authorize_user;
pub mod comments;
pub mod epics;
pub mod errors;
pub mod invitations;
pub mod issue_assignees;
pub mod issue_statuses;
pub mod issues;
pub mod messages;
pub mod models;
pub mod prelude;
pub mod projects;
pub mod schema;
pub mod tokens;
pub mod user_projects;
pub mod user_settings;
pub mod users;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub struct DbExecutor {
    pub pool: DbPool,
    pub config: jirs_config::database::Configuration,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Default for DbExecutor {
    fn default() -> Self {
        Self {
            pool: build_pool(),
            config: jirs_config::database::Configuration::read(),
        }
    }
}

pub fn build_pool() -> DbPool {
    dotenv::dotenv().ok();
    let config = jirs_config::database::Configuration::read();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    r2d2::Pool::builder()
        .max_size(config.concurrency as u32)
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create pool. {}", e))
}

pub trait SyncQuery {
    type Result;

    fn handle(&self, pool: &DbPool) -> Self::Result;
}

pub struct Guard<'l> {
    conn: &'l crate::DbPooledConn,
    tm: &'l diesel::connection::AnsiTransactionManager,
}

impl<'l> Guard<'l> {
    pub fn new(conn: &'l DbPooledConn) -> Result<Self, crate::DatabaseError> {
        use diesel::connection::TransactionManager;
        use diesel::prelude::*;
        let tm = conn.transaction_manager();
        tm.begin_transaction(conn).map_err(|e| {
            log::error!("{:?}", e);
            crate::DatabaseError::DatabaseConnectionLost
        })?;
        Ok(Self { conn, tm })
    }

    pub fn run<R, F: FnOnce(&Guard) -> Result<R, crate::DatabaseError>>(
        &self,
        f: F,
    ) -> Result<R, crate::DatabaseError> {
        use diesel::connection::TransactionManager;

        let r = f(self);
        match r {
            Ok(r) => {
                self.tm.commit_transaction(self.conn).map_err(|e| {
                    log::error!("{:?}", e);
                    crate::DatabaseError::DatabaseConnectionLost
                })?;
                Ok(r)
            }
            Err(e) => {
                log::error!("{:?}", e);
                self.tm.rollback_transaction(self.conn).map_err(|e| {
                    log::error!("{:?}", e);
                    crate::DatabaseError::DatabaseConnectionLost
                })?;
                Err(e)
            }
        }
    }
}
