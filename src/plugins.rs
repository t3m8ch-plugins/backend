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
        Ok(fs::read_dir(dir)?
            .filter_map(|file| file.ok())
            .filter_map(|file| file.file_type().ok().zip(Some(file)))
            .filter(|(file_type, _)| file_type.is_file())
            .map(|(_, file)| {
                let wasm = fs::read(file.path())?;

                let mut store = Store::default();
                let module = Module::new(&store, &wasm)?;

                let mut import_object = imports! {};
                let (plugin, _) =
                    plugin::Plugin::instantiate(&mut store, &module, &mut import_object)?;
                let manifest = plugin.get_manifest(&mut store)?;

                Ok(Plugin {
                    wasm: plugin,
                    manifest,
                })
            })
            .collect())
    }
}
