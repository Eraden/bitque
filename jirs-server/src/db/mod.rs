use std::fs::*;

use actix::{Actor, SyncContext};
#[cfg(not(debug_assertions))]
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
use crate::db::dev::VerboseConnection;

pub mod authorize_user;
pub mod comments;
pub mod issue_assignees;
pub mod issues;
pub mod projects;
pub mod tokens;
pub mod users;

#[cfg(debug_assertions)]
pub type DbPool = r2d2::Pool<ConnectionManager<dev::VerboseConnection>>;
#[cfg(debug_assertions)]
pub type DbPooledConn = r2d2::PooledConnection<ConnectionManager<dev::VerboseConnection>>;
#[cfg(not(debug_assertions))]
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[cfg(not(debug_assertions))]
pub type DbPooledConn = r2d2::PooledConnection<ConnectionManager<dev::PgConnection>>;

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

    #[cfg(not(debug_assertions))]
    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());
    #[cfg(debug_assertions)]
    let manager: ConnectionManager<VerboseConnection> =
        ConnectionManager::<dev::VerboseConnection>::new(config.database_url.as_str());
    r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create pool. {}", e))
}

pub trait SyncQuery {
    type Result;

    fn handle(&self, pool: &DbPool) -> Self::Result;
}

#[cfg(debug_assertions)]
pub mod dev {
    use std::ops::Deref;

    use diesel::connection::{AnsiTransactionManager, SimpleConnection};
    use diesel::debug_query;
    use diesel::deserialize::QueryableByName;
    use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
    use diesel::sql_types::HasSqlType;
    use diesel::{Connection, ConnectionResult, PgConnection, QueryResult, Queryable};

    pub struct VerboseConnection {
        inner: PgConnection,
    }

    impl Deref for VerboseConnection {
        type Target = PgConnection;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl SimpleConnection for VerboseConnection {
        fn batch_execute(&self, query: &str) -> QueryResult<()> {
            self.inner.batch_execute(query)
        }
    }

    impl Connection for VerboseConnection {
        type Backend = diesel::pg::Pg;
        type TransactionManager = AnsiTransactionManager;

        fn establish(database_url: &str) -> ConnectionResult<Self> {
            PgConnection::establish(database_url).map(|inner| Self { inner })
        }

        fn execute(&self, query: &str) -> QueryResult<usize> {
            self.inner.execute(query)
        }

        fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
        where
            T: AsQuery,
            T::Query: QueryFragment<Self::Backend> + QueryId,
            Self::Backend: HasSqlType<T::SqlType>,
            U: Queryable<T::SqlType, Self::Backend>,
        {
            self.inner.query_by_index(source)
        }

        fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
        where
            T: QueryFragment<Self::Backend> + QueryId,
            U: QueryableByName<Self::Backend>,
        {
            let q = debug_query::<Self::Backend, _>(&source).to_string();
            debug!("{:?}", q);
            self.inner.query_by_name(source)
        }

        fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
        where
            T: QueryFragment<Self::Backend> + QueryId,
        {
            let q = debug_query::<Self::Backend, _>(&source).to_string();
            debug!("{:?}", q);
            self.inner.execute_returning_count(source)
        }

        fn transaction_manager(&self) -> &Self::TransactionManager {
            self.inner.transaction_manager()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub concurrency: usize,
    pub database_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        let database_url = if cfg!(test) {
            "postgres://postgres@localhost:5432/jirs_test"
        } else {
            "postgres://postgres@localhost:5432/jirs"
        }
        .to_string();
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
