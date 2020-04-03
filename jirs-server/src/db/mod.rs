use actix::{Actor, SyncContext};
#[cfg(not(debug_assertions))]
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::dev::VerboseConnection;

pub mod authorize_user;
pub mod comments;
pub mod issues;
pub mod projects;
pub mod users;

#[cfg(debug_assertions)]
pub type DbPool = r2d2::Pool<ConnectionManager<dev::VerboseConnection>>;
#[cfg(not(debug_assertions))]
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbExecutor(pub DbPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new() -> Self {
        Self(build_pool())
    }
}

pub fn build_pool() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    #[cfg(not(debug_assertions))]
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    #[cfg(debug_assertions)]
    let manager: ConnectionManager<VerboseConnection> =
        ConnectionManager::<dev::VerboseConnection>::new(database_url.clone());
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
