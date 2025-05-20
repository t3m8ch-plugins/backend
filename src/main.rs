use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use anyhow::Context;
use plugins::Plugin;
use serde::Serialize;
use std::env;
use std::sync::Arc;

pub mod plugins;

struct AppState {
    plugins: Arc<Vec<Plugin>>,
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
    let plugin = data
        .plugins
        .iter()
        .filter(|p| p.manifest.name == name)
        .next();
    match plugin {
        Some(plugin) => HttpResponse::Ok().json(PluginManifestJson {
            name: plugin.manifest.name.clone(),
            description: plugin.manifest.description.clone(),
            version: PluginVersion {
                major: plugin.manifest.version.major,
                minor: plugin.manifest.version.minor,
                patch: plugin.manifest.version.patch,
            },
        }),
        None => HttpResponse::NotFound().body("Plugin not found"),
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let plugins_dir = env::var("PLUGINS_DIR")
        .with_context(|| "Failed to read PLUGINS_DIR environment variable")?;

    let plugins: Arc<Vec<plugins::Plugin>> = Arc::new(
        plugins::Plugin::load_from_dir(&plugins_dir)?
            .into_iter()
            .filter_map(|plugin| match plugin {
                Ok(plugin) => Some(plugin),
                Err(err) => {
                    log::warn!("Failed to load plugin: {}", err);
                    None
                }
            })
            .collect(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                plugins: plugins.clone(),
            }))
            .service(plugin_manifest)
    })
    .bind(("localhost", 8000))?
    .run()
    .await?;

    Ok(())
}
