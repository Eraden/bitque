use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub mod authorize_user;
pub mod issues;
pub mod projects;
pub mod users;

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
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create pool. {}", e))
}

pub trait SyncQuery {
    type Result;

    fn handle(&self, pool: &DbPool) -> Self::Result;
}
