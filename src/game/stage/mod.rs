pub mod loader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub layout: Vec<String>,
}

