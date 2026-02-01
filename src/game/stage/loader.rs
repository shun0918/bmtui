use super::StageConfig;

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
