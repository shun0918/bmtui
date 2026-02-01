use super::StageConfig;
use std::fs;
use std::path::Path;

pub fn load_stage<P: AsRef<Path>>(path: P) -> Result<StageConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: StageConfig = ron::from_str(&content)?;
    config.validate()?;
    Ok(config)
}

pub fn get_default_stage() -> StageConfig {
    StageConfig {
        name: "Stage 1".to_string(),
        width: 15,
        height: 11,
        layout: vec![
            "###############".to_string(),
            "#P   X   X   E#".to_string(),
            "# # # # # # # #".to_string(),
            "#   X   X   X #".to_string(),
            "# # # # # # # #".to_string(),
            "#   X   X   X #".to_string(),
            "# # # # # # # #".to_string(),
            "#   X   X   X #".to_string(),
            "# # # # # # # #".to_string(),
            "#E  X   X   E #".to_string(),
            "###############".to_string(),
        ],
    }
}
