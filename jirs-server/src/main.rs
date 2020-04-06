#![feature(async_closure)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod ws;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("JIRS_SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let bind = std::env::var("JIRS_SERVER_BIND").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:{}", bind, port);

    let db_addr = actix::SyncArbiter::start(4, || crate::db::DbExecutor::new());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::default())
            .data(db_addr.clone())
            .data(crate::db::build_pool())
            .service(crate::ws::index)
            .service(
                web::scope("/issues")
                    .service(crate::routes::issues::project_issues)
                    .service(crate::routes::issues::issue_with_users_and_comments)
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
            .service(web::scope("/currentUser").service(crate::routes::users::current_user))
            .service(
                web::scope("/project")
                    .service(crate::routes::projects::project_with_users_and_issues)
                    .service(crate::routes::projects::update),
            )
    })
    .bind(addr)
    .map_err(|e| format!("{}", e))?
    .run()
    .await
    .expect("Server internal error");
    Ok(())
}
