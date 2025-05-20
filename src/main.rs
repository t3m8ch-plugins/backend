use std::env;
use std::fs;
use wai_bindgen_wasmer::import;
use wasmer::{Module, Store, imports};

import!("./wai/plugin.wai");

fn main() -> anyhow::Result<()> {
    let plugins_dir = env::var("PLUGINS_DIR")?;
    let paths = fs::read_dir(plugins_dir)?;

    for file in paths {
        let file = file?;
        let file_type = file.file_type()?;

        if file_type.is_file() {
            let wasm = fs::read(file.path())?;

            let mut store = Store::default();
            let module = Module::new(&store, &wasm)?;

            let mut import_object = imports! {};
            let (plugin, _) = plugin::Plugin::instantiate(&mut store, &module, &mut import_object)?;
            let manifest = plugin.get_manifest(&mut store)?;

            println!("{:?}", manifest);
        }
    }

    Ok(())
}
