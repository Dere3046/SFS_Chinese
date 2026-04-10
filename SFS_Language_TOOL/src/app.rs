use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::language::AppLanguage;
use crate::file_manager::{FileManager, LanguageSettingsData, BackupFileInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LanguageSettings {
    code_name: String,
    custom: bool,
}

impl From<&LanguageSettingsData> for LanguageSettings {
    fn from(data: &LanguageSettingsData) -> Self {
        Self {
            code_name: data.code_name.clone(),
            custom: data.custom,
        }
    }
}

impl From<&LanguageSettings> for LanguageSettingsData {
    fn from(settings: &LanguageSettings) -> Self {
        Self {
            code_name: settings.code_name.clone(),
            custom: settings.custom,
        }
    }
}

impl Default for LanguageSettings {
    fn default() -> Self {
        Self {
            code_name: "English".to_string(),
            custom: false,
        }
    }
}

pub struct SfsLanguageApp {
    language: AppLanguage,
    game_dir: Option<PathBuf>,
    file_manager: FileManager,
    lang_settings: LanguageSettings,
    custom_files: Vec<String>,
    selected_custom_file: Option<String>,
    new_file_name: String,
    status_message: String,
    official_languages: Vec<(String, String)>,
    found_game_dirs: Vec<PathBuf>,
    show_found_dirs: bool,
    show_about: bool,
    show_key_compare: bool,
    compare_result: Option<(Vec<String>, Vec<String>)>,
    download_url: String,
    download_filename: String,
    selected_font: Option<String>,
    available_fonts: Vec<String>,
    custom_backups: Vec<BackupFileInfo>,
    settings_backups: Vec<BackupFileInfo>,
    rename_old_name: String,
    rename_new_name: String,
    show_rename_input: bool,
}

impl SfsLanguageApp {
    pub fn new() -> Self {
        let sys_lang = detect_system_language();
        let language = sys_lang;
        
        let official_languages = vec![
            ("English".to_string(), "English".to_string()),
            ("Brazilian".to_string(), "Português (Brasil)".to_string()),
            ("Chinese".to_string(), "简体中文".to_string()),
            ("Dutch".to_string(), "Nederlands".to_string()),
            ("French".to_string(), "Français".to_string()),
            ("German".to_string(), "Deutsch".to_string()),
            ("Hindi".to_string(), "हिन्दी".to_string()),
            ("Indonesian".to_string(), "Bahasa Indonesia".to_string()),
            ("Italian".to_string(), "Italiano".to_string()),
            ("Japanese".to_string(), "日本語".to_string()),
            ("Korean".to_string(), "한국어".to_string()),
            ("Malay".to_string(), "Bahasa Melayu".to_string()),
            ("Polish".to_string(), "Polski".to_string()),
            ("Russian".to_string(), "Русский".to_string()),
            ("Spanish".to_string(), "Español".to_string()),
            ("Turkish".to_string(), "Türkçe".to_string()),
            ("Czech".to_string(), "Čeština".to_string()),
            ("Hungarian".to_string(), "Magyar".to_string()),
            ("Romanian".to_string(), "Română".to_string()),
            ("Ukrainian".to_string(), "Українська".to_string()),
            ("Greek".to_string(), "Ελληνικά".to_string()),
        ];
        
        Self {
            language,
            game_dir: None,
            file_manager: FileManager::new(),
            lang_settings: LanguageSettings::default(),
            custom_files: Vec::new(),
            selected_custom_file: None,
            new_file_name: String::new(),
            status_message: String::new(),
            official_languages,
            found_game_dirs: Vec::new(),
            show_found_dirs: false,
            show_about: false,
            show_key_compare: false,
            compare_result: None,
            download_url: String::new(),
            download_filename: String::new(),
            selected_font: None,
            available_fonts: Vec::new(),
            custom_backups: Vec::new(),
            settings_backups: Vec::new(),
            rename_old_name: String::new(),
            rename_new_name: String::new(),
            show_rename_input: false,
        }
    }
    
    fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }
    
    fn is_valid_game_dir(&self, dir: &PathBuf) -> bool {
        dir.join("Spaceflight Simulator_Data").exists()
    }
    
    fn find_game_dirs(&self, start_dir: &PathBuf, max_depth: u8) -> Vec<PathBuf> {
        let mut results = Vec::new();
        self.search_dir(start_dir, max_depth, 0, &mut results);
        results
    }
    
    fn search_dir(&self, dir: &PathBuf, max_depth: u8, current_depth: u8, results: &mut Vec<PathBuf>) {
        if current_depth >= max_depth {
            return;
        }
        
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if path.join("Spaceflight Simulator_Data").exists() {
                        results.push(path.clone());
                    } else {
                        self.search_dir(&path, max_depth, current_depth + 1, results);
                    }
                }
            }
        }
    }
    
    fn load_fonts(&mut self) {
        self.available_fonts = self.file_manager.get_available_fonts();
    }
    
    fn load_backups(&mut self) {
        self.custom_backups = self.file_manager.list_backup_files();
        self.settings_backups = self.file_manager.list_settings_backups();
    }
    
    fn format_size(size: u64) -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        }
    }
}

impl eframe::App for SfsLanguageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button(self.language.app_title()).clicked() {
                    let current = self.language.clone();
                    let new_lang = match current {
                        AppLanguage::Chinese => AppLanguage::English,
                        AppLanguage::English => AppLanguage::Chinese,
                    };
                    self.language = new_lang;
                }
                
                ui.separator();
                ui.label(format!("{}: {}", self.language.language_label(), self.language.display_name()));
                
                ui.separator();
                if ui.button(self.language.btn_about()).clicked() {
                    self.show_about = true;
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let lang = self.language.clone();
            ui.heading(lang.app_title());
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing(lang.section_game_dir(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(lang.label_game_dir());
                    if let Some(dir) = &self.game_dir {
                        ui.label(dir.to_string_lossy().to_string());
                    } else {
                        ui.label(lang.label_no_dir());
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button(lang.btn_select_dir()).clicked() {
                        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                            if self.is_valid_game_dir(&dir) {
                                self.game_dir = Some(dir.clone());
                                self.file_manager.set_game_dir(&dir);
                                self.custom_files = self.file_manager.list_custom_translation_files();
                                if let Some(settings) = self.file_manager.load_language_settings() {
                                    self.lang_settings = LanguageSettings::from(&settings);
                                }
                                self.load_fonts();
                                let msg = lang.status_dir_loaded();
                                self.set_status(msg);
                            } else {
                                let msg = lang.error_invalid_dir();
                                self.set_status(msg);
                            }
                        }
                    }
                    
                    if ui.button(lang.btn_auto_find()).clicked() {
                        if let Some(start_dir) = rfd::FileDialog::new().pick_folder() {
                            let found = self.find_game_dirs(&start_dir, 4);
                            if found.is_empty() {
                                let msg = lang.error_no_game_found();
                                self.set_status(msg);
                            } else {
                                self.found_game_dirs = found;
                                self.show_found_dirs = true;
                            }
                        }
                    }
                });
                
                if self.show_found_dirs && !self.found_game_dirs.is_empty() {
                    ui.separator();
                    ui.label(lang.label_found_dirs());
                    let found_dirs = self.found_game_dirs.clone();
                    for dir in &found_dirs {
                        ui.horizontal(|ui| {
                            if ui.button(dir.to_string_lossy().to_string()).clicked() {
                                self.game_dir = Some(dir.clone());
                                self.file_manager.set_game_dir(dir);
                                self.custom_files = self.file_manager.list_custom_translation_files();
                                if let Some(settings) = self.file_manager.load_language_settings() {
                                    self.lang_settings = LanguageSettings::from(&settings);
                                }
                                self.load_fonts();
                                self.show_found_dirs = false;
                                let msg = lang.status_dir_loaded();
                                self.set_status(msg);
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        if ui.button(lang.btn_clear_found()).clicked() {
                            self.show_found_dirs = false;
                            self.found_game_dirs.clear();
                        }
                    });
                }
            });
            
            ui.separator();
            
            ui.collapsing(lang.section_custom_files(), |ui| {
                ui.label(lang.desc_custom_files());
                
                ui.separator();
                ui.label(lang.label_download_from_url());
                ui.horizontal(|ui| {
                    ui.label(lang.label_url());
                    ui.text_edit_singleline(&mut self.download_url);
                });
                ui.horizontal(|ui| {
                    ui.label(lang.label_filename());
                    ui.text_edit_singleline(&mut self.download_filename);
                });
                if ui.button(lang.btn_download()).clicked() {
                    if self.download_url.is_empty() {
                        let msg = lang.error_empty_url();
                        self.set_status(msg);
                    } else if self.download_filename.is_empty() || !self.download_filename.ends_with(".txt") {
                        let msg = lang.error_invalid_filename();
                        self.set_status(msg);
                    } else {
                        match self.file_manager.download_custom_file(&self.download_url, &self.download_filename) {
                            Ok(_) => {
                                self.custom_files = self.file_manager.list_custom_translation_files();
                                let msg = lang.status_download_success();
                                self.set_status(msg);
                                self.download_url.clear();
                                self.download_filename.clear();
                            }
                            Err(e) => {
                                let msg = format!("{}: {}", lang.error_download_failed(), e);
                                self.set_status(msg);
                            }
                        }
                    }
                }
                
                ui.separator();
                
                if !self.custom_files.is_empty() {
                    ui.label(lang.label_existing_files());
                    let mut selected_file = self.selected_custom_file.clone();
                    let custom_files = self.custom_files.clone();
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for file in &custom_files {
                            let is_selected = selected_file.as_deref() == Some(file);
                            ui.horizontal(|ui| {
                                if ui.selectable_label(is_selected, file).clicked() {
                                    selected_file = Some(file.clone());
                                }
                                
                                if ui.button(lang.btn_edit()).clicked() {
                                    self.file_manager.open_in_notepad(file);
                                }
                                
                                if ui.button(lang.btn_backup()).clicked() {
                                    match self.file_manager.backup_custom_file(file) {
                                        Ok(name) => {
                                            let msg = format!("{}: {}", lang.status_backup_success(), name);
                                            self.set_status(msg);
                                        }
                                        Err(e) => {
                                            let msg = format!("{}: {}", lang.error_backup_failed(), e);
                                            self.set_status(msg);
                                        }
                                    }
                                }

                                if ui.button(lang.btn_restore()).clicked() {
                                    let target = file.clone();
                                    if self.file_manager.restore_custom_file(&format!("{}.bak", file), &target) {
                                        let msg = lang.status_restore_success();
                                        self.set_status(msg);
                                    } else {
                                        let msg = lang.error_restore_failed();
                                        self.set_status(msg);
                                    }
                                }
                                
                                if ui.button(lang.btn_delete()).clicked() {
                                    if self.file_manager.delete_custom_file(file) {
                                        self.custom_files.retain(|f| f != file);
                                        if self.selected_custom_file.as_deref() == Some(file) {
                                            self.selected_custom_file = None;
                                        }
                                        let msg = lang.status_file_deleted();
                                        self.set_status(msg);
                                    }
                                }
                            });
                        }
                    });
                    self.selected_custom_file = selected_file;
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(lang.btn_compare_keys()).clicked() {
                            if let Some(selected) = &self.selected_custom_file {
                                let result = self.file_manager.compare_keys(selected);
                                self.compare_result = Some(result);
                                self.show_key_compare = true;
                            } else {
                                let msg = lang.error_no_file_selected();
                                self.set_status(msg);
                            }
                        }
                    });
                } else {
                    ui.label(lang.label_no_files());
                }
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label(lang.label_new_filename());
                    ui.text_edit_singleline(&mut self.new_file_name);
                    
                    if ui.button(lang.btn_create()).clicked() {
                        if self.new_file_name.is_empty() {
                            let msg = lang.error_empty_filename();
                            self.set_status(msg);
                        } else if !self.new_file_name.ends_with(".txt") {
                            let msg = lang.error_no_txt_extension();
                            self.set_status(msg);
                        } else if self.custom_files.contains(&self.new_file_name) {
                            let msg = lang.error_file_exists();
                            self.set_status(msg);
                        } else if self.game_dir.is_some() {
                            let file_name = self.new_file_name.clone();
                            if self.file_manager.create_custom_file(&file_name) {
                                self.custom_files.push(file_name.clone());
                                self.new_file_name.clear();
                                let msg = lang.status_file_created();
                                self.set_status(msg);
                            } else {
                                let msg = lang.error_create_file();
                                self.set_status(msg);
                            }
                        } else {
                            let msg = lang.error_no_dir();
                            self.set_status(msg);
                        }
                    }
                });
                
                ui.separator();
                ui.label(lang.label_font_config());
                
                ui.label(lang.font_config_warning());
                
                if self.available_fonts.is_empty() {
                    self.load_fonts();
                }
                
                if self.available_fonts.is_empty() {
                    ui.label(lang.label_no_fonts_found());
                } else {
                    let mut current_idx = 0;
                    if let Some(selected) = &self.selected_font {
                        for (i, font) in self.available_fonts.iter().enumerate() {
                            if font == selected {
                                current_idx = i;
                                break;
                            }
                        }
                    }
                    
                    egui::ComboBox::from_label(lang.label_select_font())
                        .selected_text(
                            self.available_fonts.get(current_idx)
                                .cloned()
                                .unwrap_or_else(|| lang.label_select_font().to_string())
                        )
                        .show_ui(ui, |ui| {
                            for (i, font) in self.available_fonts.iter().enumerate() {
                                if ui.selectable_label(current_idx == i, font).clicked() {
                                    current_idx = i;
                                    self.selected_font = Some(font.clone());
                                }
                            }
                        });
                    
                    ui.horizontal(|ui| {
                        if ui.button(lang.btn_apply_font()).clicked() {
                            if let Some(font) = &self.selected_font {
                                if let Some(file) = &self.selected_custom_file {
                                    if self.file_manager.set_font_in_file(file, font) {
                                        let msg = lang.status_font_applied();
                                        self.set_status(msg);
                                    } else {
                                        let msg = lang.error_font_apply_failed();
                                        self.set_status(msg);
                                    }
                                } else {
                                    let msg = lang.error_no_file_selected();
                                    self.set_status(msg);
                                }
                            } else {
                                let msg = lang.error_no_font_selected();
                                self.set_status(msg);
                            }
                        }
                    });
                }
            });

            ui.separator();

            ui.collapsing(lang.section_backup_manager(), |ui| {
                ui.label(lang.desc_backup_manager());
                
                ui.horizontal(|ui| {
                    if ui.button(lang.btn_refresh_backups()).clicked() {
                        self.load_backups();
                    }
                });
                
                ui.separator();
                ui.label(lang.label_custom_backups());
                
                if self.custom_backups.is_empty() {
                    ui.label(lang.label_no_backups());
                } else {
                    let backups = self.custom_backups.clone();
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        for backup in &backups {
                            ui.horizontal(|ui| {
                                ui.label(format!("{} ({}, {})", 
                                    backup.filename, 
                                    backup.modified_time,
                                    Self::format_size(backup.size)));
                                
                                if ui.button(lang.btn_restore()).clicked() {
                                    let target = backup.original_name.trim_end_matches(".bak").to_string() 
                                        + ".txt";
                                    if self.file_manager.restore_custom_file(&backup.filename, &target) {
                                        let msg = lang.status_restore_success();
                                        self.set_status(msg);
                                    } else {
                                        let msg = lang.error_restore_failed();
                                        self.set_status(msg);
                                    }
                                }
                                
                                if ui.button(lang.btn_rename()).clicked() {
                                    self.rename_old_name = backup.filename.clone();
                                    self.rename_new_name = backup.filename.clone();
                                    self.show_rename_input = true;
                                }
                                
                                if ui.button(lang.btn_delete()).clicked() {
                                    if self.file_manager.delete_backup_file(&backup.filename) {
                                        self.load_backups();
                                        let msg = lang.status_file_deleted();
                                        self.set_status(msg);
                                    }
                                }
                            });
                        }
                    });
                }
                
                if self.show_rename_input {
                    ui.separator();
                    ui.label(lang.label_rename_backup());
                    ui.horizontal(|ui| {
                        ui.label(lang.label_new_filename());
                        ui.text_edit_singleline(&mut self.rename_new_name);
                        
                        if ui.button(lang.btn_confirm()).clicked() {
                            if self.rename_new_name.ends_with(".bak") && !self.rename_new_name.is_empty() {
                                if self.file_manager.rename_backup_file(&self.rename_old_name, &self.rename_new_name) {
                                    self.show_rename_input = false;
                                    self.load_backups();
                                    let msg = lang.status_rename_success();
                                    self.set_status(msg);
                                } else {
                                    let msg = lang.error_rename_failed();
                                    self.set_status(msg);
                                }
                            } else {
                                let msg = lang.error_invalid_bak_name();
                                self.set_status(msg);
                            }
                        }
                        
                        if ui.button(lang.btn_cancel()).clicked() {
                            self.show_rename_input = false;
                        }
                    });
                }
                
                ui.separator();
                ui.label(lang.label_settings_backups());
                
                if self.settings_backups.is_empty() {
                    ui.label(lang.label_no_backups());
                } else {
                    let backups = self.settings_backups.clone();
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for backup in &backups {
                            ui.horizontal(|ui| {
                                ui.label(format!("{} ({}, {})", 
                                    backup.filename, 
                                    backup.modified_time,
                                    Self::format_size(backup.size)));
                                
                                if ui.button(lang.btn_restore()).clicked() {
                                    if self.file_manager.restore_language_settings(&backup.filename) {
                                        let msg = lang.status_restore_success();
                                        self.set_status(msg);
                                        if let Some(settings) = self.file_manager.load_language_settings() {
                                            self.lang_settings = LanguageSettings::from(&settings);
                                        }
                                    } else {
                                        let msg = lang.error_restore_failed();
                                        self.set_status(msg);
                                    }
                                }
                                
                                if ui.button(lang.btn_delete()).clicked() {
                                    if self.file_manager.delete_backup_file(&backup.filename) {
                                        self.load_backups();
                                        let msg = lang.status_file_deleted();
                                        self.set_status(msg);
                                    }
                                }
                            });
                        }
                    });
                }
            });

            ui.separator();

            ui.collapsing(lang.section_bepinex(), |ui| {
                ui.label(lang.desc_bepinex());

                let bepinex_installed = self.file_manager.is_bepinex_installed();
                let font_fix_installed = self.file_manager.is_font_fix_installed();
                let installed_version = self.file_manager.get_installed_font_fix_version();

                ui.separator();
                ui.label("状态:");

                ui.horizontal(|ui| {
                    if bepinex_installed {
                        ui.colored_label(egui::Color32::GREEN, lang.status_bepinex_installed());
                    } else {
                        ui.colored_label(egui::Color32::YELLOW, lang.status_bepinex_not_installed());
                    }
                });

                ui.horizontal(|ui| {
                    if font_fix_installed {
                        let version_text = installed_version.as_deref().unwrap_or("unknown");
                        ui.colored_label(
                            egui::Color32::GREEN,
                            format!("{} (v{})", lang.status_font_fix_installed(), version_text)
                        );
                    } else {
                        ui.colored_label(egui::Color32::YELLOW, lang.status_font_fix_not_installed());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("{}: v{}", lang.label_current_version(),
                        crate::file_manager::FileManager::PLUGIN_VERSION));
                });

                ui.separator();

                // BepInEx 安装/卸载按钮
                ui.horizontal(|ui| {
                    if !bepinex_installed {
                        if ui.button(lang.btn_install_bepinex()).clicked() {
                            if self.file_manager.install_bepinex(None) {
                                let msg = lang.status_bepinex_install_success();
                                self.set_status(msg);
                            } else {
                                let msg = lang.error_bepinex_not_found();
                                self.set_status(msg);
                            }
                        }
                    } else {
                        if ui.button(lang.btn_uninstall_bepinex()).clicked() {
                            if self.file_manager.uninstall_bepinex() {
                                let msg = lang.status_bepinex_uninstall_success();
                                self.set_status(msg);
                            } else {
                                let msg = lang.error_uninstall_failed();
                                self.set_status(msg);
                            }
                        }
                    }
                });

                // 字体修复插件按钮
                if bepinex_installed {
                    ui.separator();
                    ui.horizontal(|ui| {
                        if font_fix_installed {
                            if ui.button(lang.btn_update_font_fix()).clicked() {
                                if self.file_manager.install_font_fix_plugin() {
                                    let msg = lang.status_font_fix_updated();
                                    self.set_status(msg);
                                } else {
                                    let msg = lang.error_install_failed();
                                    self.set_status(msg);
                                }
                            }

                            if ui.button(lang.btn_uninstall_font_fix()).clicked() {
                                if self.file_manager.uninstall_font_fix_plugin() {
                                    let msg = lang.status_uninstall_success();
                                    self.set_status(msg);
                                } else {
                                    let msg = lang.error_uninstall_failed();
                                    self.set_status(msg);
                                }
                            }
                        } else {
                            if ui.button(lang.btn_install_font_fix()).clicked() {
                                if self.file_manager.install_font_fix_plugin() {
                                    let msg = lang.status_font_fix_installed_msg();
                                    self.set_status(msg);
                                } else {
                                    let msg = lang.error_install_failed();
                                    self.set_status(msg);
                                }
                            }
                        }
                    });
                }
            });

            ui.separator();

            ui.collapsing(lang.section_lang_settings(), |ui| {
                ui.label(lang.desc_lang_settings());

                ui.horizontal(|ui| {
                    if ui.button(lang.btn_backup_settings()).clicked() {
                        match self.file_manager.backup_language_settings() {
                            Ok(name) => {
                                let msg = format!("{}: {}", lang.status_backup_success(), name);
                                self.set_status(msg);
                            }
                            Err(e) => {
                                let msg = format!("{}: {}", lang.error_backup_failed(), e);
                                self.set_status(msg);
                            }
                        }
                    }

                    if ui.button(lang.btn_restore_settings()).clicked() {
                        if let Some(backup) = self.settings_backups.first() {
                            if self.file_manager.restore_language_settings(&backup.filename) {
                                let msg = lang.status_restore_success();
                                self.set_status(msg);
                                if let Some(settings) = self.file_manager.load_language_settings() {
                                    self.lang_settings = LanguageSettings::from(&settings);
                                }
                            } else {
                                let msg = lang.error_restore_failed();
                                self.set_status(msg);
                            }
                        } else {
                            let msg = lang.error_no_backup_found();
                            self.set_status(msg);
                        }
                    }
                });
                
                ui.separator();
                
                ui.checkbox(&mut self.lang_settings.custom, lang.checkbox_use_custom());
                
                if self.lang_settings.custom {
                    ui.label(lang.label_select_custom());
                    
                    if self.custom_files.is_empty() {
                        ui.label(lang.label_no_files());
                    } else {
                        egui::ComboBox::from_label(lang.label_select_file())
                            .selected_text(self.selected_custom_file.as_deref().unwrap_or(lang.label_select_file()))
                            .show_ui(ui, |ui| {
                                for file in &self.custom_files {
                                    ui.selectable_value(
                                        &mut self.selected_custom_file,
                                        Some(file.clone()),
                                        file
                                    );
                                }
                            });
                    
                        if let Some(selected) = &self.selected_custom_file {
                            let code_name = selected.trim_end_matches(".txt");
                            self.lang_settings.code_name = code_name.to_string();
                        }
                    }
                } else {
                    ui.label(lang.label_select_official());
                    
                    let mut current_idx = 0;
                    for (i, (code, _)) in self.official_languages.iter().enumerate() {
                        if code == &self.lang_settings.code_name {
                            current_idx = i;
                            break;
                        }
                    }
                    
                    egui::ComboBox::from_label(lang.label_language())
                        .selected_text(
                            self.official_languages.get(current_idx)
                                .map(|(_, name)| name.clone())
                                .unwrap_or_else(|| "English".to_string())
                        )
                        .show_ui(ui, |ui| {
                            for (i, (code, name)) in self.official_languages.iter().enumerate() {
                                if ui.selectable_label(current_idx == i, name).clicked() {
                                    current_idx = i;
                                    self.lang_settings.code_name = code.clone();
                                }
                            }
                        });
                }
                
                ui.separator();
                
                if ui.button(lang.btn_save_settings()).clicked() {
                    if self.game_dir.is_some() {
                        let settings_data = LanguageSettingsData::from(&self.lang_settings);
                        if self.file_manager.save_language_settings(&settings_data) {
                            let msg = lang.status_settings_saved();
                            self.set_status(msg);
                        } else {
                            let msg = lang.error_save_settings();
                            self.set_status(msg);
                        }
                    } else {
                        let msg = lang.error_no_dir();
                        self.set_status(msg);
                    }
                }
                
                ui.separator();
                ui.label(lang.label_settings_preview());
                ui.monospace(&format!(
                    "{{\n  \"codeName\": \"{}\",\n  \"custom\": {}\n}}",
                    self.lang_settings.code_name,
                    self.lang_settings.custom
                ));
            });
            
            ui.separator();
            
            if !self.status_message.is_empty() {
                ui.colored_label(egui::Color32::YELLOW, &self.status_message);
            }
            });
        });
        
        if self.show_about {
            egui::Window::new(self.language.about_title())
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(self.language.about_developer());
                    ui.label(self.language.about_project());
                    
                    if ui.button("https://github.com/Dere3046/SFS_LANG_TOOL").clicked() {
                        let _ = open::that("https://github.com/Dere3046/SFS_LANG_TOOL");
                    }
                    
                    ui.separator();
                    ui.label(self.language.about_license_title());
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        ui.monospace(self.language.about_license_content());
                    });
                    
                    ui.separator();
                    if ui.button(self.language.btn_close()).clicked() {
                        self.show_about = false;
                    }
                });
        }
        
        if self.show_key_compare {
            egui::Window::new(self.language.compare_title())
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    if let Some((missing, extra)) = &self.compare_result {
                        ui.label(self.language.compare_missing());
                        if missing.is_empty() {
                            ui.colored_label(egui::Color32::GREEN, self.language.compare_none());
                        } else {
                            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                                for key in missing {
                                    ui.monospace(key);
                                }
                            });
                        }
                        
                        ui.separator();
                        ui.label(self.language.compare_extra());
                        if extra.is_empty() {
                            ui.colored_label(egui::Color32::GREEN, self.language.compare_none());
                        } else {
                            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                                for key in extra {
                                    ui.monospace(key);
                                }
                            });
                        }
                    }
                    
                    ui.separator();
                    if ui.button(self.language.btn_close()).clicked() {
                        self.show_key_compare = false;
                    }
                });
        }
    }
}

fn detect_system_language() -> AppLanguage {
    if let Some(locale) = sys_locale::get_locale() {
        if locale.starts_with("zh") || locale.starts_with("ZH") {
            return AppLanguage::Chinese;
        }
    }
    AppLanguage::English
}
