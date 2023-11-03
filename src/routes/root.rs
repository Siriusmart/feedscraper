use actix_web::{get, http::header::ContentType, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::ROOT_HTML;

#[get("/")]
pub async fn root_service() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(ROOT_HTML.get().unwrap().clone())
}

#[derive(Serialize, Deserialize)]
struct Res {
    error: String,
}
