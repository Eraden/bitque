#![feature(async_closure)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix::Actor;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};

use crate::ws::WsServer;

pub mod db;
pub mod errors;
pub mod mail;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod utils;
pub mod web;
pub mod ws;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let web_config = web::Configuration::read();

    let db_addr = actix::SyncArbiter::start(
        crate::db::Configuration::read().concurrency,
        crate::db::DbExecutor::default,
    );
    let mail_addr = actix::SyncArbiter::start(
        crate::mail::Configuration::read().concurrency,
        crate::mail::MailExecutor::default,
    );

    let ws_server = WsServer::default().start();

    HttpServer::new(move || {
        let web_config = web::Configuration::read();
        let mut app = App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::default())
            .data(ws_server.clone())
            .data(db_addr.clone())
            .data(mail_addr.clone())
            .data(crate::db::build_pool())
            .service(crate::ws::index)
            .service(actix_web::web::scope("/avatar").service(crate::web::avatar::upload));
        if let Some(file_system) = web_config.filesystem.as_ref() {
            app = app.service(fs::Files::new(
                file_system.client_path.as_str(),
                file_system.store_path.as_str(),
            ));
        }
        app
    })
    .workers(web_config.concurrency)
    .bind(web_config.addr())
    .map_err(|e| format!("{}", e))?
    .run()
    .await
    .expect("Server internal error");
    Ok(())
}
