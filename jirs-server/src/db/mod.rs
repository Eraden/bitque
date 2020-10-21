use std::fs::*;

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceErrors;

pub mod authorize_user;
pub mod comments;
pub mod epics;
pub mod invitations;
pub mod issue_assignees;
pub mod issue_statuses;
pub mod issues;
pub mod messages;
pub mod projects;
pub mod tokens;
pub mod user_projects;
pub mod users;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub struct DbExecutor {
    pub pool: DbPool,
    pub config: Configuration,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Default for DbExecutor {
    fn default() -> Self {
        Self {
            pool: build_pool(),
            config: Configuration::read(),
        }
    }
}

pub fn build_pool() -> DbPool {
    dotenv::dotenv().ok();
    let config = Configuration::read();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create pool. {}", e))
}

pub trait SyncQuery {
    type Result;

    fn handle(&self, pool: &DbPool) -> Self::Result;
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub database_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        let database_url = if cfg!(test) {
            "postgres://postgres@localhost:5432/jirs_test".to_string()
        } else {
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres@localhost:5432/jirs".to_string())
        };
        Self {
            concurrency: 2,
            database_url,
        }
    }
}

impl Configuration {
    pub fn read() -> Self {
        let contents: String = read_to_string(Self::config_file()).unwrap_or_default();
        match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            _ => {
                let config = Configuration::default();
                config.write().unwrap_or_else(|e| panic!(e));
                config
            }
        }
    }

    pub fn write(&self) -> Result<(), String> {
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    pub fn config_file() -> &'static str {
        "db.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "db.test.toml"
    }
}

#[macro_export]
macro_rules! db_pool {
    ($self: expr) => {
        &$self.pool.get().map_err(|e| {
            error!("{:?}", e);
            ServiceErrors::DatabaseConnectionLost
        })?
    };
}

#[macro_export]
macro_rules! q {
    ($q: expr) => {{
        let q = $q;
        debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&q).to_string()
        );
        q
    }};
}

pub struct Guard<'l> {
    conn: &'l crate::db::DbPooledConn,
    tm: &'l diesel::connection::AnsiTransactionManager,
}

impl<'l> Guard<'l> {
    pub fn new(conn: &'l DbPooledConn) -> Result<Self, ServiceErrors> {
        use diesel::{connection::TransactionManager, prelude::*};
        let tm = conn.transaction_manager();
        tm.begin_transaction(conn).map_err(|e| {
            log::error!("{:?}", e);
            ServiceErrors::DatabaseConnectionLost
        })?;
        Ok(Self { conn, tm })
    }

    pub fn run<R, F: FnOnce(&Guard) -> Result<R, ServiceErrors>>(
        &self,
        f: F,
    ) -> Result<R, ServiceErrors> {
        use diesel::connection::TransactionManager;

        let r = f(self);
        match r {
            Ok(r) => {
                self.tm.commit_transaction(self.conn).map_err(|e| {
                    log::error!("{:?}", e);
                    ServiceErrors::DatabaseConnectionLost
                })?;
                Ok(r)
            }
            Err(e) => {
                log::error!("{:?}", e);
                self.tm.rollback_transaction(self.conn).map_err(|e| {
                    log::error!("{:?}", e);
                    ServiceErrors::DatabaseConnectionLost
                })?;
                Err(e)
            }
        }
    }
}
