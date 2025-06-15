use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    actix_state::AppState,
    api::dto::{PluginManifestJson, PluginVersion},
};

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
