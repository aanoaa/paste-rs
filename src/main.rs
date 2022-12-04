use std::time::Duration;
use std::{env, path::PathBuf};
use std::{fs, thread};

use actix_web::{App, HttpServer};
use paste_rs::util;
use paste_rs::{config::Config, server, CONFIG_ENV};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let config_path = match env::var(CONFIG_ENV).ok() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("config.toml"),
    };

    let config = Config::from(&config_path).expect("config read fail");
    log::trace!("{:#?}", config);

    if config.delete_expired_enabled.unwrap_or(false) {
        thread::spawn(move || loop {
            log::info!("delete expired files job started");
            let ttl = match config.delete_expired_ttl {
                Some(dur) => dur.as_secs(),
                None => Duration::from_secs(60 * 60 * 24).as_secs(),
            };
            match util::expired_files(&config.upload_path, ttl) {
                Ok(expired) => {
                    for file in expired {
                        match fs::remove_file(&file) {
                            Ok(()) => log::info!("expired file removed: {:?}", file),
                            Err(e) => log::error!("remove expired file fail: {e}"),
                        }
                    }
                }
                Err(e) => {
                    log::error!("expired files fail: {e}");
                }
            }

            log::info!("delete expired files job finished");
            thread::sleep(Duration::from_secs(3600)); // interval 1h
        });
    }

    HttpServer::new(|| App::new().configure(server::configure))
        .bind(config.address)?
        .run()
        .await
}
