use std::{path::PathBuf, sync::Arc};

use crate::config::Config;

pub struct AppState {
    pub config: Arc<Config>,
    pub base_dir: PathBuf,
    pub web_dir: PathBuf,
}
