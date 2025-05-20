import!("./wai/plugin.wai");

use std::fs;

use wai_bindgen_wasmer::import;
use wasmer::{Module, Store, imports};

pub struct Plugin {
    pub manifest: plugin::Manifest,
    wasm: plugin::Plugin,
}

impl Plugin {
    pub fn load_from_dir(dir: &str) -> anyhow::Result<Vec<Plugin>> {
        let mut plugins: Vec<Plugin> = vec![];

        let paths = fs::read_dir(dir)?;
        for file in paths {
            let file = file?;
            let file_type = file.file_type()?;

            if file_type.is_file() {
                let wasm = fs::read(file.path())?;

                let mut store = Store::default();
                let module = Module::new(&store, &wasm)?;

                let mut import_object = imports! {};
                let (plugin, _) =
                    plugin::Plugin::instantiate(&mut store, &module, &mut import_object)?;

                let manifest = plugin.get_manifest(&mut store)?;
                plugins.push(Plugin {
                    wasm: plugin,
                    manifest,
                });
            }
        }

        Ok(plugins)
    }
}
