import!("./wai/plugin.wai");

use std::{collections::HashMap, fs};

use serde::Serialize;
use wai_bindgen_wasmer::import;
use wasmer::{Module, Store, imports};

pub struct Plugin {
    pub manifest: plugin::Manifest,
    wasm: plugin::Plugin,
    store: Store,
}

#[derive(Debug, Clone, Serialize)]
pub struct UiNode {
    name: String,
    props: HashMap<String, String>,
    children: Vec<UiNode>,
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
                log::debug!(
                    "Loaded plugin manifest: {} v{}.{}.{}",
                    manifest.name,
                    manifest.version.major,
                    manifest.version.minor,
                    manifest.version.patch
                );

                Ok(Plugin {
                    wasm: plugin,
                    store: store,
                    manifest,
                })
            })
            .collect())
    }

    pub fn get_ui(&mut self) -> anyhow::Result<UiNode> {
        let wasm_ui_tree = self.wasm.get_ui_tree(&mut self.store)?;

        if wasm_ui_tree.nodes.is_empty() {
            return Err(anyhow::anyhow!("UI tree is empty!"));
        }

        Ok(self.build_ui_node(0, &wasm_ui_tree))
    }

    fn build_ui_node(&self, node_idx: usize, wasm_ui_tree: &plugin::UiTree) -> UiNode {
        let wasm_node = &wasm_ui_tree.nodes[node_idx];
        let children = wasm_ui_tree
            .children
            .get(node_idx)
            .map(|child_indicies| {
                child_indicies
                    .iter()
                    .map(|&child_idx| self.build_ui_node(child_idx as usize, wasm_ui_tree))
                    .collect()
            })
            .unwrap_or_default();

        UiNode {
            name: wasm_node.name.clone(),
            props: wasm_node.props.clone().into_iter().collect(),
            children,
        }
    }
}
