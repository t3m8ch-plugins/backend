use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use anyhow::Context;
use config::Config;
use plugins::Plugin;
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;

pub mod config;
pub mod plugins;

struct AppState {
    plugins: Arc<Mutex<Vec<Plugin>>>,
}

#[derive(Serialize)]
struct PluginVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Serialize)]
struct PluginManifestJson {
    name: String,
    description: String,
    version: PluginVersion,
}

#[get("/plugins/{name}/manifest")]
async fn plugin_manifest(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = name.into_inner();
    log::info!("Requesting manifest for plugin: {}", name);

    let plugins = data.plugins.lock().unwrap();
    let plugin = plugins.iter().filter(|p| p.manifest.name == name).next();
    match plugin {
        Some(plugin) => {
            log::debug!("Found plugin manifest for: {}", name);
            HttpResponse::Ok().json(PluginManifestJson {
                name: plugin.manifest.name.clone(),
                description: plugin.manifest.description.clone(),
                version: PluginVersion {
                    major: plugin.manifest.version.major,
                    minor: plugin.manifest.version.minor,
                    patch: plugin.manifest.version.patch,
                },
            })
        }
        None => {
            log::warn!("Plugin not found: {}", name);
            HttpResponse::NotFound().body("Plugin not found")
        }
    }
}

#[get("/plugins/{name}/ui")]
async fn plugin_ui(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = name.into_inner();
    log::info!("Requesting UI for plugin: {}", name);

    let mut plugins = data.plugins.lock().unwrap();
    let plugin = plugins.iter_mut().find(|p| p.manifest.name == name);

    match plugin {
        Some(plugin) => {
            log::debug!("Found plugin {}", name);
            let ui = plugin.get_ui().unwrap();
            HttpResponse::Ok().json(ui)
        }
        None => {
            log::warn!("Plugin not found: {}", name);
            HttpResponse::NotFound().body("Plugin not found")
        }
    }
}

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
            .service(plugin_manifest)
            .service(plugin_ui)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    log::info!("Server stopped");
    Ok(())
}
