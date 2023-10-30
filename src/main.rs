use actix_web::{App, HttpServer};
use feedscraper::*;
use scrapyard::{Feeds, Saveable};

#[tokio::main]
async fn main() {
    let config_path = dirs::config_dir().unwrap().join(env!("CARGO_PKG_NAME"));
    scrapyard::init(Some(&config_path)).await;
    FScraperConfig::init().await;

    let feeds_path = config_path.join("feeds.json");
    let feeds = if feeds_path.exists() {
        Feeds::load_json(&feeds_path).await.unwrap()
    } else {
        let default = Feeds::default();
        default.save_json_pretty(&feeds_path).await.unwrap();
        default
    };

    feedscraper::init(feeds.clone());
    feeds.start_loop().await;

    let mut server = HttpServer::new(|| {
        let mut app = App::new()
            .service(feedinfo_service)
            .service(rss_service)
            .service(feedhome_service)
            .service(meta_service)
            .service(root_service);

        if FS_CONFIG.get().unwrap().show_feed_configs {
            app = app.service(config_service);
        }

        app
    });

    let config = FS_CONFIG.get().unwrap();

    if let Some(port) = config.http_port {
        server = server.bind(("0.0.0.0", port)).unwrap()
    }

    if let Some(https) = &config.https {
        server = server
            .bind_rustls(
                ("0.0.0.0", https.port),
                load_rustls_config(&https.chain, &https.key),
            )
            .unwrap()
    }

    println!("Server started");
    server.run().await.unwrap()
}
