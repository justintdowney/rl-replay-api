#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;
use std::sync::Arc;
use serde::Serialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, middleware, web::Data};
use awc::{http::header, Client, Connector};
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}
fn init_logger() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Error, Config::default(), File::create("replay-api.log").unwrap()),
        ]
    ).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let replay_db = repository::database::Database::new();
    let app_data = web::Data::new(replay_db);
    init_logger();
    HttpServer::new(move || {
        Client::builder()
        .add_default_header((header::USER_AGENT, "replay-api/1.0"))
        .finish();

        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(health_check)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
