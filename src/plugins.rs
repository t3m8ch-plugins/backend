import!("./wai/plugin.wai");

use std::fs;

use wai_bindgen_wasmer::import;
use wasmer::{Module, Store, imports};

pub struct Plugin {
    pub manifest: plugin::Manifest,
    wasm: plugin::Plugin,
}

impl Plugin {
    pub fn load_from_dir(dir: &str) -> anyhow::Result<Vec<anyhow::Result<Plugin>>> {
        log::debug!("Scanning plugins directory: {}", dir);
        Ok(fs::read_dir(dir)?
            .filter_map(|file| file.ok())
            .filter_map(|file| file.file_type().ok().zip(Some(file)))
            .filter(|(file_type, _)| file_type.is_file())
            .map(|(_, file)| {
                let file_path = file.path();
                log::debug!("Processing plugin file: {:?}", file_path);
                let wasm = fs::read(&file_path)?;

                let mut store = Store::default();
                let module = Module::new(&store, &wasm)?;
                log::debug!("Created WASM module for: {:?}", file_path);

                let mut import_object = imports! {};
                let (plugin, _) =
                    plugin::Plugin::instantiate(&mut store, &module, &mut import_object)?;
                let manifest = plugin.get_manifest(&mut store)?;
                log::debug!("Loaded plugin manifest: {} v{}.{}.{}", 
                    manifest.name, 
                    manifest.version.major, 
                    manifest.version.minor, 
                    manifest.version.patch
                );

                Ok(Plugin {
                    wasm: plugin,
                    manifest,
                })
            })
            .collect())
    }
}
