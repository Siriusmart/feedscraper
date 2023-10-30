use actix_web::{get, http::header::ContentType, web::Path, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::FEEDS_MAP;

#[get("{label}/feedinfo.json")]
pub async fn feedinfo_service(label: Path<String>) -> HttpResponse {
    match FEEDS_MAP.get().unwrap().get(label.as_str()) {
        Some(feed) => match feed.lazy_fetch_json().await {
            Ok(feed) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(feed),
            Err(e) => HttpResponse::InternalServerError().json(Res {
                error: e.to_string(),
            }),
        },
        None => HttpResponse::NotFound().json(Res {
            error: "not found".to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize)]
struct Res {
    error: String,
}
