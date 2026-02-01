pub mod loader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub layout: Vec<String>,
}

impl StageConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.layout.len() != self.height {
            return Err(format!(
                "Layout height {} doesn't match height {}",
                self.layout.len(),
                self.height
            ));
        }

        for (i, row) in self.layout.iter().enumerate() {
            if row.len() != self.width {
                return Err(format!(
                    "Layout row {} width {} doesn't match width {}",
                    i,
                    row.len(),
                    self.width
                ));
            }
        }

        Ok(())
    }
}
