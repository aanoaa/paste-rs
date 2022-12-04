use std::{env, path::PathBuf};

use actix_web::{App, HttpServer};
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

    HttpServer::new(|| App::new().configure(server::configure))
        .bind(config.address)?
        .run()
        .await
}
