use serde::Serialize;

#[derive(Serialize)]
pub struct PluginVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Serialize)]
pub struct PluginManifestJson {
    pub name: String,
    pub description: String,
    pub version: PluginVersion,
}
