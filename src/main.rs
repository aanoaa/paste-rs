use std::{env, io::Result, path::PathBuf};

use paste_rs::{config::Config, CONFIG_ENV};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let config_path = match env::var(CONFIG_ENV).ok() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("config.toml"),
    };

    let config = Config::from(&config_path).expect("config read fail");
    log::trace!("{:#?}", config);
    Ok(())
}
