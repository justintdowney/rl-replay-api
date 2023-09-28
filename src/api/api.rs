use actix_web::*;
use actix_web::dev::Payload;
use futures_util::*;
use actix_multipart::Multipart;
use awc::Client;
use crate::{models::replay::Replay, repository::database::Database};

/*
Upload replay file as a single part named file using multipart/form-data. 
This API returns 201 on success (with the id of the created replay) or 409 in case of a duplicate replay (with the id of the existing replay). 
*/
#[post("/replays")]
pub async fn put_replay(
    mut payload: Multipart, request: HttpRequest) -> impl Responder {
    // 10 MB
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10;
    const MAX_FILE_COUNT: i32 = 1;

    // detect malformed requests
    let content_length: u64 = match request.headers().get("content-length") {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };

    // reject malformed requests
    match content_length {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest()
                .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
        },
        _ => {}
    };

    let mut file_count = 0;

    while let Some(mut field) = payload.try_next().await.unwrap_or(None) {
        if let Some(filename) = field.content_disposition().get_filename() {
            if file_count == MAX_FILE_COUNT {
                return HttpResponse::BadRequest().body(format!(
                    "Too many files uploaded. Maximum count is {}.", MAX_FILE_COUNT
                ));
            }
            let replay = boxcars::ParserBuilder::new(&field)
        .must_parse_network_data()
        .on_error_check_crc()
        .parse()?;
    let data = subtr_actor::ReplayDataCollector::new()
        .get_replay_data(&replay);
        println!("{}", data.get_frame_data().balldata)
            file_count += 1;
        }
    }

    if file_count != 1 {
        return HttpResponse::BadRequest().body("Exactly one file must be uploaded.");
    }

    HttpResponse::Ok().finish()
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