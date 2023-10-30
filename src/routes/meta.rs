use actix_web::{get, web::Path, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::FEEDS_MAP;

#[get("{label}/meta.json")]
pub async fn meta_service(label: Path<String>) -> HttpResponse {
    match FEEDS_MAP.get().unwrap().get(label.as_str()) {
        Some(feed) => HttpResponse::Ok().json(feed.meta().await.unwrap()),
        None => HttpResponse::NotFound().json(Res {
            error: "not found".to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize)]
struct Res {
    error: String,
}
