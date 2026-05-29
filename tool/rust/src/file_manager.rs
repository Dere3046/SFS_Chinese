use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSettings {
    #[serde(rename = "codeName")]
    pub code_name: String,
    pub custom: bool,
}

pub struct FileManager;

impl FileManager {
    pub const MOD_VERSION: &str = "5.1.0";

    pub fn is_mod_installed(game_dir: &Path) -> bool {
        let m = game_dir.join("Mods").join("SFS_HAN_MOD");
        m.join("version.txt").exists() && m.join("SFS_HAN_MOD.dll").exists()
    }

    pub fn install_mod(game_dir: &Path) {
        let m = game_dir.join("Mods").join("SFS_HAN_MOD");
        fs::create_dir_all(&m).ok();
        let dll = include_bytes!("../assets/SFS_HAN_MOD.dll");
        let font = include_bytes!("../assets/NotoSansSC.ttf");
        fs::write(m.join("SFS_HAN_MOD.dll"), dll).ok();
        fs::write(m.join("NotoSansSC.ttf"), font).ok();
        fs::write(m.join("version.txt"), Self::MOD_VERSION).ok();
    }

    pub fn uninstall_mod(game_dir: &Path) {
        let m = game_dir.join("Mods").join("SFS_HAN_MOD");
        if m.exists() {
            fs::remove_dir_all(m).ok();
        }
    }

    pub fn set_chinese_language(game_dir: &Path) {
        let settings = game_dir
            .join("Saving")
            .join("Settings")
            .join("LanguageSettings_2.txt");
        let config = LanguageSettings {
            code_name: "Chinese".to_string(),
            custom: false,
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            fs::write(settings, json).ok();
        }
    }
}