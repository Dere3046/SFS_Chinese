use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSettingsData {
    #[serde(rename = "codeName")]
    pub code_name: String,
    pub custom: bool,
}

#[derive(Debug, Clone)]
pub struct BackupFileInfo {
    pub filename: String,
    pub original_name: String,
    pub modified_time: String,
    pub size: u64,
}

pub struct FileManager {
    game_dir: Option<PathBuf>,
}

impl FileManager {
    pub fn new() -> Self {
        Self { game_dir: None }
    }

    pub fn set_game_dir(&mut self, dir: &Path) {
        self.game_dir = Some(dir.to_path_buf());
    }

    pub const PLUGIN_VERSION: &str = "5.1.0";

    pub fn is_bepinex_installed(&self) -> bool {
        if let Some(dir) = &self.game_dir {
            dir.join("BepInEx").join("core").join("BepInEx.dll").exists()
        } else {
            false
        }
    }

    pub fn is_font_fix_installed(&self) -> bool {
        if let Some(game_dir) = &self.game_dir {
            let plugin_dir = game_dir.join("BepInEx").join("plugins").join("SFS_FontFix");
            plugin_dir.join("SFS_FontFix.dll").exists()
                && (plugin_dir.join("NotoSansSC.ttf").exists() || plugin_dir.join("NotoSansSC.otf").exists())
        } else {
            false
        }
    }

    pub fn get_installed_font_fix_version(&self) -> Option<String> {
        if let Some(game_dir) = &self.game_dir {
            let version_file = game_dir.join("BepInEx").join("plugins").join("SFS_FontFix").join("version.txt");
            if version_file.exists() {
                fs::read_to_string(&version_file).ok().map(|s| s.trim().to_string())
            } else if self.is_font_fix_installed() {
                Some("Unknown (pre-4.1)".to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn install_bepinex(&self, bepinex_source: Option<&Path>) -> bool {
        if let Some(game_dir) = &self.game_dir {
            if self.is_bepinex_installed() {
                return true;
            }

            let mut final_source: Option<PathBuf> = bepinex_source.map(|p| p.to_path_buf());
            if final_source.is_none() || !final_source.as_ref().unwrap().exists() {
                if let Some(exe_path) = std::env::current_exe().ok() {
                    if let Some(parent) = exe_path.parent() {
                        let local_folder = parent.join("BepInEx_win_x64_5.4.23.5");
                        if local_folder.exists() {
                            final_source = Some(local_folder);
                        }
                    }
                }
            }

            if let Some(src) = &final_source {
                if let Ok(entries) = fs::read_dir(src) {
                    for entry in entries.flatten() {
                        let src_path = entry.path();
                        let dst_path = game_dir.join(entry.file_name());
                        if src_path.is_dir() {
                            if copy_dir_all(&src_path, &dst_path).is_err() {
                                return false;
                            }
                        } else {
                            if fs::copy(&src_path, &dst_path).is_err() {
                                return false;
                            }
                        }
                    }
                }
            } else {
                return false;
            }

            self.install_font_fix_plugin_internal(game_dir);
            true
        } else {
            false
        }
    }

    pub fn uninstall_bepinex(&self) -> bool {
        if let Some(game_dir) = &self.game_dir {
            let mut success = true;

            let be_dir = game_dir.join("BepInEx");
            if be_dir.exists() {
                if fs::remove_dir_all(&be_dir).is_err() {
                    success = false;
                }
            }

            for file in &["winhttp.dll", "doorstop_config.ini", "UnityDoorstop.dll", "version.dll"] {
                let path = game_dir.join(file);
                if path.exists() && fs::remove_file(&path).is_err() {
                    success = false;
                }
            }

            success
        } else {
            false
        }
    }

    pub fn install_font_fix_plugin(&self) -> bool {
        if let Some(game_dir) = &self.game_dir {
            let plugin_dir = game_dir.join("BepInEx").join("plugins").join("SFS_FontFix");
            fs::create_dir_all(&plugin_dir).ok();
            self.install_font_fix_plugin_internal(game_dir);
            true
        } else {
            false
        }
    }

    fn install_font_fix_plugin_internal(&self, game_dir: &PathBuf) {
        let plugin_dir = game_dir.join("BepInEx").join("plugins").join("SFS_FontFix");
        let plugin_dll = plugin_dir.join("SFS_FontFix.dll");
        let font_file = plugin_dir.join("NotoSansSC.ttf");
        let version_file = plugin_dir.join("version.txt");

        let plugin_bytes = include_bytes!("../assets/SFS_FontFix.dll");
        let font_bytes = include_bytes!("../assets/NotoSansSC.ttf");

        fs::write(&plugin_dll, plugin_bytes).ok();
        fs::write(&font_file, font_bytes).ok();
        fs::write(&version_file, Self::PLUGIN_VERSION).ok();
    }

    pub fn uninstall_font_fix_plugin(&self) -> bool {
        if let Some(game_dir) = &self.game_dir {
            let plugin_dir = game_dir.join("BepInEx").join("plugins").join("SFS_FontFix");
            if plugin_dir.exists() {
                fs::remove_dir_all(&plugin_dir).is_ok()
            } else {
                true
            }
        } else {
            false
        }
    }

    fn get_custom_translations_dir(&self) -> Option<PathBuf> {
        self.game_dir.as_ref().map(|d| d.join("Spaceflight Simulator_Data").join("Custom Translations"))
    }

    fn get_settings_dir(&self) -> Option<PathBuf> {
        self.game_dir.as_ref().map(|d| d.join("Saving").join("Settings"))
    }

    pub fn list_custom_translation_files(&self) -> Vec<String> {
        if let Some(dir) = self.get_custom_translations_dir() {
            if dir.exists() {
                let mut files = Vec::new();
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                let name = name.to_string();
                                if name != "Example.txt" && name != "READ_ME.txt" {
                                    files.push(name);
                                }
                            }
                        }
                    }
                }
                files.sort();
                return files;
            }
        }
        Vec::new()
    }

    pub fn create_custom_file(&self, filename: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            fs::create_dir_all(&dir).ok();
            let file_path = dir.join(filename);
            if !file_path.exists() {
                return fs::write(&file_path, "[None]\n    None=None\n    Font=normal\n\n[General]\n").is_ok();
            }
        }
        false
    }

    pub fn download_custom_file(&self, url: &str, filename: &str) -> Result<String, String> {
        if let Some(dir) = self.get_custom_translations_dir() {
            fs::create_dir_all(&dir).ok();
            let file_path = dir.join(filename);
            match ureq::get(url).call() {
                Ok(response) => {
                    let text = response.into_string().map_err(|e| e.to_string())?;
                    fs::write(&file_path, text.as_bytes()).map_err(|e| e.to_string())?;
                    Ok(text)
                }
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("No game directory".to_string())
        }
    }

    pub fn open_in_notepad(&self, filename: &str) {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if file_path.exists() {
                Command::new("notepad.exe").arg(&file_path).spawn().ok();
            }
        }
    }

    pub fn delete_custom_file(&self, filename: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if file_path.exists() {
                return fs::remove_file(&file_path).is_ok();
            }
        }
        false
    }

    pub fn backup_custom_file(&self, filename: &str) -> Result<String, String> {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if !file_path.exists() {
                return Err("File not found".to_string());
            }
            let backup_name = self.generate_backup_name(&dir, &filename)?;
            let backup_path = dir.join(&backup_name);
            fs::copy(&file_path, &backup_path).map(|_| backup_name).map_err(|e| e.to_string())
        } else {
            Err("No game directory".to_string())
        }
    }

    pub fn restore_custom_file(&self, backup_filename: &str, target_filename: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            let backup_path = dir.join(backup_filename);
            let file_path = dir.join(target_filename);
            if backup_path.exists() {
                return fs::copy(&backup_path, &file_path).is_ok();
            }
        }
        false
    }

    pub fn rename_backup_file(&self, old_name: &str, new_name: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            let old_path = dir.join(old_name);
            let new_path = dir.join(new_name);
            if old_path.exists() && new_name.ends_with(".bak") {
                return fs::rename(&old_path, &new_path).is_ok();
            }
        }
        false
    }

    pub fn delete_backup_file(&self, filename: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if file_path.exists() {
                return fs::remove_file(&file_path).is_ok();
            }
        }
        false
    }

    pub fn list_backup_files(&self) -> Vec<BackupFileInfo> {
        let mut backups = Vec::new();
        if let Some(dir) = self.get_custom_translations_dir() {
            if dir.exists() {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |e| e == "bak") {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                let original = name.trim_end_matches(".bak").to_string();
                                let modified = Self::get_file_modified_time(&path);
                                let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                                backups.push(BackupFileInfo { filename: name.to_string(), original_name: original, modified_time: modified, size });
                            }
                        }
                    }
                }
            }
        }
        backups.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));
        backups
    }

    pub fn backup_language_settings(&self) -> Result<String, String> {
        if let Some(dir) = self.get_settings_dir() {
            let file_path = dir.join("LanguageSettings_2.txt");
            if !file_path.exists() {
                return Err("Settings file not found".to_string());
            }
            let backup_name = self.generate_settings_backup_name(&dir)?;
            let backup_path = dir.join(&backup_name);
            fs::copy(&file_path, &backup_path).map(|_| backup_name).map_err(|e| e.to_string())
        } else {
            Err("No game directory".to_string())
        }
    }

    pub fn restore_language_settings(&self, backup_filename: &str) -> bool {
        if let Some(dir) = self.get_settings_dir() {
            let backup_path = dir.join(backup_filename);
            let file_path = dir.join("LanguageSettings_2.txt");
            if backup_path.exists() {
                return fs::copy(&backup_path, &file_path).is_ok();
            }
        }
        false
    }

    pub fn list_settings_backups(&self) -> Vec<BackupFileInfo> {
        let mut backups = Vec::new();
        if let Some(dir) = self.get_settings_dir() {
            if dir.exists() {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |e| e == "bak") {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                let modified = Self::get_file_modified_time(&path);
                                let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                                backups.push(BackupFileInfo { filename: name.to_string(), original_name: "LanguageSettings_2.txt".to_string(), modified_time: modified, size });
                            }
                        }
                    }
                }
            }
        }
        backups.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));
        backups
    }

    fn generate_backup_name(&self, dir: &PathBuf, filename: &str) -> Result<String, String> {
        let base_name = format!("{}.bak", filename);
        let base_path = dir.join(&base_name);
        if !base_path.exists() {
            return Ok(base_name);
        }
        for i in 1..100 {
            let name = format!("{}.{}.bak", filename, i);
            let path = dir.join(&name);
            if !path.exists() {
                return Ok(name);
            }
        }
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
        Ok(format!("{}.{}.bak", filename, timestamp))
    }

    fn generate_settings_backup_name(&self, dir: &PathBuf) -> Result<String, String> {
        let base_name = "LanguageSettings_2.txt.bak".to_string();
        let base_path = dir.join(&base_name);
        if !base_path.exists() {
            return Ok(base_name);
        }
        for i in 1..100 {
            let name = format!("LanguageSettings_2.txt.{}.bak", i);
            let path = dir.join(&name);
            if !path.exists() {
                return Ok(name);
            }
        }
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
        Ok(format!("LanguageSettings_2.txt.{}.bak", timestamp))
    }

    fn get_file_modified_time(path: &PathBuf) -> String {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let datetime: chrono::DateTime<chrono::Local> = chrono::DateTime::from(modified);
                return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            }
        }
        "Unknown".to_string()
    }

    pub fn load_language_settings(&self) -> Option<LanguageSettingsData> {
        if let Some(dir) = self.get_settings_dir() {
            let file_path = dir.join("LanguageSettings_2.txt");
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Ok(settings) = serde_json::from_str::<LanguageSettingsData>(&content) {
                        return Some(settings);
                    }
                }
            }
        }
        None
    }

    pub fn save_language_settings(&self, settings: &LanguageSettingsData) -> bool {
        if let Some(dir) = self.get_settings_dir() {
            fs::create_dir_all(&dir).ok();
            let file_path = dir.join("LanguageSettings_2.txt");
            if let Ok(json) = serde_json::to_string_pretty(settings) {
                return fs::write(&file_path, json).is_ok();
            }
        }
        false
    }

    pub fn get_example_keys(&self) -> HashSet<String> {
        let mut keys = HashSet::new();
        if let Some(dir) = self.get_custom_translations_dir() {
            let example_path = dir.join("Example.txt");
            if example_path.exists() {
                if let Ok(content) = fs::read_to_string(&example_path) {
                    keys = self.parse_keys(&content);
                }
            }
        }
        keys
    }

    pub fn get_file_keys(&self, filename: &str) -> HashSet<String> {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    return self.parse_keys(&content);
                }
            }
        }
        HashSet::new()
    }

    pub fn compare_keys(&self, filename: &str) -> (Vec<String>, Vec<String>) {
        let example_keys = self.get_example_keys();
        let file_keys = self.get_file_keys(filename);
        let skip_keys = vec!["[None].Font", "[General].Font", "Font", "None"];
        let missing: Vec<String> = example_keys.difference(&file_keys).filter(|k| !skip_keys.contains(&k.as_str())).cloned().collect();
        let extra: Vec<String> = file_keys.difference(&example_keys).filter(|k| !skip_keys.contains(&k.as_str())).cloned().collect();
        (missing, extra)
    }

    fn parse_keys(&self, content: &str) -> HashSet<String> {
        let mut keys = HashSet::new();
        let mut current_section = String::new();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
            } else if line.contains('=') && !line.starts_with('#') {
                if let Some(key) = line.split('=').next() {
                    let full_key = if current_section.is_empty() { key.trim().to_string() } else { format!("[{}].{}", current_section, key.trim()) };
                    keys.insert(full_key);
                }
            }
        }
        keys
    }

    pub fn get_available_fonts(&self) -> Vec<String> {
        vec!["normal".to_string()]
    }

    pub fn set_font_in_file(&self, filename: &str, font_name: &str) -> bool {
        if let Some(dir) = self.get_custom_translations_dir() {
            let file_path = dir.join(filename);
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                    let mut found = false;
                    for (i, line) in lines.iter().enumerate() {
                        let trimmed = line.trim();
                        if trimmed == "Font" || (trimmed.starts_with("Font") && trimmed.contains('=')) {
                            lines[i] = format!("    Font={}", font_name);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if let Some(idx) = lines.iter().position(|l| l.trim() == "[None]") {
                            lines.insert(idx + 1, format!("    Font={}", font_name));
                        } else {
                            lines.insert(0, "[None]".to_string());
                            lines.insert(1, "    None=None".to_string());
                            lines.insert(2, format!("    Font={}", font_name));
                            lines.insert(3, "".to_string());
                        }
                    }
                    return fs::write(&file_path, lines.join("\n")).is_ok();
                }
            }
        }
        false
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
