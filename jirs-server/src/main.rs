#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

pub mod db;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let db_addr = actix::SyncArbiter::start(4, || crate::db::DbExecutor::new());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::default())
            .data(db_addr.clone())
            .service(
                web::scope("/issues")
                    .wrap(crate::middleware::authorize::Authorize::default())
                    .service(crate::routes::issues::project_issues)
                    .service(crate::routes::issues::issue_with_users_and_omments)
                    .service(crate::routes::issues::create)
                    .service(crate::routes::issues::update)
                    .service(crate::routes::issues::delete),
            )
            .service(
                web::scope("/comments")
                    .service(crate::routes::comments::create)
                    .service(crate::routes::comments::update)
                    .service(crate::routes::comments::delete),
            )
    })
    .bind("127.0.0.1:8080")
    .map_err(|e| format!("{}", e))?
    .run()
    .await
    .expect("Server internal error");
    Ok(())
}
