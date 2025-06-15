use std::sync::{Arc, Mutex};

use crate::plugins::Plugin;

pub struct AppState {
    pub plugins: Arc<Mutex<Vec<Plugin>>>,
}
