use std::{path::PathBuf, fs};

use actix_web::{*, http::Error};
use actix_web::dev::Payload;
use futures_util::*;
use actix_multipart::{form::{MultipartForm, tempfile::TempFile}, Multipart};
use awc::Client;
use serde_with::SerializeAs;
use crate::{models::{replay::Replay, remote_defs::BallFrameDef}, repository::database::Database};

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}


/*
Upload replay file as a single part named file using multipart/form-data. 
This API returns 201 on success (with the id of the created replay) or 409 in case of a duplicate replay (with the id of the existing replay). 
*/
#[post("/replays")]
pub async fn put_replay(
    form: MultipartForm<Upload>) -> Result<HttpResponse, Error> {
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB
    const MAX_FILE_COUNT: i32 = 1;

    match form.file.size {
        0 => Err("The file size is zero."),
        length if length as u64 > MAX_FILE_SIZE => {
            Err("The uploaded file is too large. Maximum size is {} bytes.")
        },
    };
    
    let data = std::fs::read(form.file.file.path()).unwrap();
    let replay = boxcars::ParserBuilder::new(&data)
    .must_parse_network_data()
    .on_error_check_crc()
    .parse();

    let parsed_replay = 
    subtr_actor::ReplayDataCollector::new()
        .get_replay_data(&replay.unwrap())
        .unwrap();

    let temp_file_path = form.file.file.path();
    let file_name: &str = form
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let mut file_path = PathBuf::from("./output/");
    //file_path.push(&sanitize_filename::sanitize(&file_name));

    match std::fs::rename(temp_file_path, file_path) {
        Err(f) => f,
    };

    Ok(HttpResponse::Ok().json(&parsed_replay))
}




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(put_replay)
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
    .limit(20_000_000)// limit 3mb
    .await
    .unwrap();

    let replay = boxcars::ParserBuilder::new(&data)
    .must_parse_network_data()
    .on_error_check_crc()
    .parse();

    replay.unwrap().properties
}