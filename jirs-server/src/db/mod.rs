use std::fs::*;

use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

pub mod authorize_user;
pub mod comments;
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
