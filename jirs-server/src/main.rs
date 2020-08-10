#![feature(async_closure)]
#![feature(vec_remove_item)]
#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix::Actor;
use actix_cors::Cors;
#[cfg(feature = "local-storage")]
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

    std::fs::create_dir_all(web_config.tmp_dir.as_str()).map_err(|e| e.to_string())?;
    #[cfg(feature = "local-storage")]
    if !web_config.filesystem.is_empty() {
        let filesystem = &web_config.filesystem;
        std::fs::create_dir_all(filesystem.store_path.as_str()).map_err(|e| e.to_string())?;
    }

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
        let app = App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::default())
            .data(ws_server.clone())
            .data(db_addr.clone())
            .data(mail_addr.clone())
            .data(crate::db::build_pool())
            .service(crate::ws::index)
            .service(actix_web::web::scope("/avatar").service(crate::web::avatar::upload));

        #[cfg(feature = "local-storage")]
        let web_config = web::Configuration::read();
        #[cfg(feature = "local-storage")]
        let app = if !web_config.filesystem.is_empty() {
            let filesystem = &web_config.filesystem;
            app.service(fs::Files::new(
                filesystem.client_path.as_str(),
                filesystem.store_path.as_str(),
            ))
        } else {
            app
        };
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
