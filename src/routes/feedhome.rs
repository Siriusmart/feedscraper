use actix_web::{http::header::ContentType, routes, web::Path, HttpResponse};
use chrono::Utc;
use scrapyard::{PseudoItemCache, Saveable, MASTER};

use crate::{FEEDS_MAP, FS_CONFIG};

#[routes]
#[get("/{label}")]
#[get("/{label}/")]
pub async fn feedhome_service(label: Path<String>) -> HttpResponse {
    match FEEDS_MAP.get().unwrap().get(label.as_str()) {
        Some(feed) => {
            let meta = feed.meta().await.unwrap();

            let title = html_escape::encode_text(&feed.channel.title);

            let label = html_escape::encode_text(&feed.label);
            let desc = html_escape::encode_text(&feed.channel.description);
            let max = feed.max_length;
            let link = html_escape::encode_text(&feed.channel.link);

            let cache_path = MASTER
                .get()
                .unwrap()
                .store
                .join(&feed.label)
                .join("cache.json");
            let cached = PseudoItemCache::load_json(&cache_path).await.unwrap();

            let cached = cached.0.len();
            let time = duration(Utc::now().timestamp() as u64 - meta.last_fetch);
            let idle = if feed.idle(&meta) { " (idle)" } else { "" };

            let config = if FS_CONFIG.get().unwrap().show_feed_configs {
                format!(r#" <a href="/{label}/config.json">Config</a>"#)
            } else {
                String::new()
            };

            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>{title}</title>
</head>
<body>
    <h1>{title}</h1>
    <p>{desc}</p>
    <ul>
        <li>Last fetched: {time} ago{idle}</li>
        <li>Items count: {cached} (cached) out of {max} (max)</li>
    </ul>
    <span>User urls: <a href="{link}">Source</a> <a href="/{label}/rss.xml">RSS</a></span><br />
    <span>Developer resources: <a href="/{label}/feedinfo.json">JSON</a> <a href="/{label}/meta.json">Meta</a>{config}</span>
</body>
</html>"#
            );
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html)
        }
        None => HttpResponse::NotFound().body("Feed not found"),
    }
}

const DURATIONS: &[(&str, u64)] = &[("day", 86400), ("hour", 3600), ("minute", 60)];

pub fn duration(secs: u64) -> String {
    for (s, n) in DURATIONS.iter().copied() {
        if secs >= n {
            let count = secs / n;
            if n == 1 {
                return format!("around a {s}");
            } else {
                return format!("around {count} {s}s");
            }
        }
    }

    "less than a minute".to_string()
}
