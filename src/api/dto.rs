use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventFromFrontend {
    pub event: String,
    pub text_input_states: HashMap<String, String>,
}
