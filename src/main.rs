use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use anyhow::Context;
use config::Config;
use std::sync::Arc;
use std::sync::Mutex;

use crate::actix_state::AppState;
use crate::api::plugins::plugin_manifest;
use crate::api::plugins::plugin_ui;

pub mod actix_state;
pub mod api;
pub mod config;
pub mod plugins;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    let config = Config::from_env().with_context(|| "Failed to load configuration")?;

    unsafe {
        std::env::set_var("RUST_LOG", &config.log_level);
    }
    env_logger::init();

    log::info!("Starting backend server with config: {:?}", config);
    log::info!("Loading plugins from directory: {}", config.plugins_dir);

    let plugins: Arc<Mutex<Vec<plugins::Plugin>>> = Arc::new(Mutex::new(
        plugins::Plugin::load_from_dir(&config.plugins_dir)?
            .into_iter()
            .filter_map(|plugin| match plugin {
                Ok(plugin) => {
                    log::info!("Successfully loaded plugin: {}", plugin.manifest.name);
                    Some(plugin)
                }
                Err(err) => {
                    log::warn!("Failed to load plugin: {}", err);
                    None
                }
            })
            .collect(),
    ));

    log::info!("Loaded {} plugins total", plugins.lock().unwrap().len());

    let server_addr = config.addr();
    log::info!(
        "Starting HTTP server on {}:{}",
        server_addr.0,
        server_addr.1
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                plugins: plugins.clone(),
            }))
            .wrap(Cors::permissive())
            .service(plugin_manifest)
            .service(plugin_ui)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    log::info!("Server stopped");
    Ok(())
}
