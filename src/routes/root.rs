use actix_web::{get, http::header::ContentType, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

use crate::FEEDS_LIST;

#[get("/")]
pub async fn root_service() -> HttpResponse {
    let feeds = FEEDS_LIST
        .get()
        .unwrap()
        .iter()
        .fold(String::new(), |mut s, (label, title)| {
            write!(
                s,
                r#"<li><a href="/{}">{}</a></li>"#,
                html_escape::encode_text(label),
                html_escape::encode_text(title)
            )
            .unwrap();
            s
        });
    // .map(|(label, title)| {
    //     format!(
    //         r#"<li><a href="/{}">{}</a></li>"#,
    //         html_escape::encode_text(label),
    //         html_escape::encode_text(title)
    //     )
    // })
    // .collect::<String>();
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>Feedscraper</title>
</head>
<body>
    <h1>Welcome to Feedscraper</h1>
    <p>Feedscraper is an automatic and scriptable RSS generater. <a href="https://github.com/siriusmart/feedscraper">Source</a></p>
    <h2>Available feeds</h2>
    <ul>{feeds}</ul>
</body>
</html>"#
    );
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[derive(Serialize, Deserialize)]
struct Res {
    error: String,
}
