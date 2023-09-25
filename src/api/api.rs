use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, HttpResponse};
use awc::Client;
use crate::{models::replay::Replay, repository::database::Database};

/*
Upload replay file as a single part named file using multipart/form-data. 
This API returns 201 on success (with the id of the created replay) or 409 in case of a duplicate replay (with the id of the existing replay). 
*/
#[post("/replays")]
pub async fn create_replay(db: Data<Database>, url: String) -> HttpResponse {
    println!("{}", url);
    let headers: Vec<(String, boxcars::HeaderProp)> = download_replay_from_url(&url).await;
    /* 
    let replay = db.create_replay(new_replay.into_inner());
    match replay {
        Ok(replay) => HttpResponse::Ok().json(replay),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }*/
    HttpResponse::Ok().json(headers)
}

/* 
#[post("/replays")]
pub async fn create_replay(db: Data<Database>, new_replay: Json<Replay>) -> HttpResponse {
    let replay = db.create_replay(new_replay.into_inner());
    match replay {
        Ok(replay) => HttpResponse::Ok().json(replay),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}*/


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_replay)
    );
}

async fn download_replay_from_url(url: &str) -> Vec<(String, boxcars::HeaderProp)>
{
    let client = Client::default();

    let mut res = client
    .get(url)
    .send()
    .await
    .unwrap();

    let data = res
    .body()
    .limit(3_000_000)// limit 3mb
    .await
    .unwrap();

    let replay = boxcars::ParserBuilder::new(&data)
    .must_parse_network_data()
    .on_error_check_crc()
    .parse();

    replay.unwrap().properties
}