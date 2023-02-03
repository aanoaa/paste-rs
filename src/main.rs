use std::time::Duration;
use std::{env, path::PathBuf};
use std::{fs, thread};

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use paste_rs::{config::Config, server, CONFIG_ENV};
use paste_rs::{util, DEFAULT_WORKERS};

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

    if !&config.upload_path.exists() {
        fs::create_dir_all(&config.upload_path)?;
    }

    let delete_expired_enabled = config.delete_expired_enabled.unwrap_or(false);
    let cleanup_config = config.clone();
    if delete_expired_enabled {
        thread::spawn(move || loop {
            log::info!("delete expired files job started");
            let ttl = match cleanup_config.delete_expired_ttl {
                Some(dur) => dur.as_secs(),
                None => Duration::from_secs(60 * 60 * 24).as_secs(),
            };
            match util::expired_files(&cleanup_config.upload_path, ttl) {
                Ok(expired) => {
                    for file in expired {
                        match fs::remove_file(&file) {
                            Ok(()) => log::info!("expired file removed: {:?}", file),
                            Err(e) => log::error!("remove expired file fail: {e}"),
                        }
                    }
                }
                Err(e) => {
                    log::error!("list expired files fail: {e}");
                }
            }

            log::info!("delete expired files job finished");
            thread::sleep(Duration::from_secs(3600)); // interval 1h
        });
    }

    let config_data = Data::new(config.clone());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default().exclude("health").exclude("/favicon.ico"))
            .app_data(Data::clone(&config_data))
            .configure(server::configure)
    })
    .bind(config.address)?
    .workers(config.workers.unwrap_or(DEFAULT_WORKERS))
    .run()
    .await
}
