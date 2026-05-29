use eframe::egui;
use std::path::PathBuf;
use crate::file_manager::FileManager;

pub struct SfsLanguageApp {
    game_dir: Option<PathBuf>,
    mod_installed: bool,
    status_message: String,
    show_about: bool,
    found_dirs: Vec<PathBuf>,
    show_found_dirs: bool,
}

impl SfsLanguageApp {
    pub fn new() -> Self {
        Self {
            game_dir: None,
            mod_installed: false,
            status_message: String::new(),
            show_about: false,
            found_dirs: Vec::new(),
            show_found_dirs: false,
        }
    }

    fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    fn update_mod_status(&mut self) {
        if let Some(dir) = &self.game_dir {
            self.mod_installed = FileManager::is_mod_installed(dir);
        } else {
            self.mod_installed = false;
        }
    }

    fn is_valid_game_dir(dir: &PathBuf) -> bool {
        dir.join("Spaceflight Simulator.exe").exists() || dir.join("Spaceflight Simulator_Data").exists()
    }

    fn find_game_dirs(start_dir: &PathBuf, max_depth: u32) -> Vec<PathBuf> {
        let mut results = Vec::new();
        Self::search_dir(start_dir, 0, max_depth, &mut results);
        results
    }

    fn search_dir(dir: &PathBuf, depth: u32, max_depth: u32, results: &mut Vec<PathBuf>) {
        if depth > max_depth {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if Self::is_valid_game_dir(&path) {
                        results.push(path);
                    } else {
                        Self::search_dir(&path, depth + 1, max_depth, results);
                    }
                }
            }
        }
    }

    fn license_text() -> &'static str {
        r#"MIT License

Copyright (c) 2026 Dere and WMYYL

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#
    }
}

impl eframe::App for SfsLanguageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("SFS 语言工具");
                ui.separator();
                if ui.button("关于").clicked() {
                    self.show_about = true;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("游戏目录");
            ui.horizontal(|ui| {
                ui.label("路径:");
                if let Some(dir) = &self.game_dir {
                    ui.label(dir.display().to_string());
                } else {
                    ui.label("未选择");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("手动选择").clicked() {
                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                        if Self::is_valid_game_dir(&dir) {
                            self.game_dir = Some(dir.clone());
                            self.update_mod_status();
                            self.set_status("游戏目录已加载");
                        } else {
                            self.set_status("所选目录不是有效的游戏目录（缺少 Spaceflight Simulator.exe 或 Spaceflight Simulator_Data）");
                        }
                    }
                }

                if ui.button("自动查找").clicked() {
                    if let Some(start_dir) = rfd::FileDialog::new().pick_folder() {
                        self.set_status("正在搜索，请稍候...");
                        let found = Self::find_game_dirs(&start_dir, 4);
                        if found.is_empty() {
                            self.set_status("在选定目录及其子目录中未找到游戏");
                        } else {
                            self.found_dirs = found;
                            self.show_found_dirs = true;
                            self.set_status(&format!("找到 {} 个游戏目录，请选择一个", self.found_dirs.len()));
                        }
                    }
                }
            });

            if self.show_found_dirs && !self.found_dirs.is_empty() {
                ui.separator();
                ui.label("找到的游戏目录（点击选择）:");
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    let dirs = self.found_dirs.clone();
                    for dir in dirs {
                        if ui.button(dir.display().to_string()).clicked() {
                            self.game_dir = Some(dir);
                            self.update_mod_status();
                            self.show_found_dirs = false;
                            self.set_status("游戏目录已加载");
                        }
                    }
                });
                if ui.button("关闭列表").clicked() {
                    self.show_found_dirs = false;
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("MOD 状态:");
                if self.mod_installed {
                    ui.colored_label(egui::Color32::GREEN, format!("已安装 (版本 {})", FileManager::MOD_VERSION));
                } else {
                    ui.colored_label(egui::Color32::YELLOW, "未安装");
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("一键设置中文").clicked() {
                    if let Some(dir) = &self.game_dir {
                        FileManager::set_chinese_language(dir);
                        self.set_status("语言已设置为中文");
                    } else {
                        self.set_status("请先选择游戏目录");
                    }
                }

                let mod_btn_text = if self.mod_installed {
                    "卸载 MOD"
                } else {
                    "安装 MOD"
                };
                if ui.button(mod_btn_text).clicked() {
                    if let Some(dir) = &self.game_dir {
                        if self.mod_installed {
                            FileManager::uninstall_mod(dir);
                            self.set_status("MOD 已卸载");
                        } else {
                            FileManager::install_mod(dir);
                            self.set_status("MOD 已安装");
                        }
                        self.update_mod_status();
                    } else {
                        self.set_status("请先选择游戏目录");
                    }
                }
            });

            ui.separator();

            if !self.status_message.is_empty() {
                ui.colored_label(egui::Color32::LIGHT_BLUE, &self.status_message);
            }
        });

        if self.show_about {
            egui::Window::new("关于")
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("SFS Language Tool v2.0.0");
                    ui.label("开发者: Dere3046, WMYYL (我没有油了)");
                    ui.hyperlink_to("项目地址: SFS_Chinese", "https://github.com/Dere3046/SFS_Chinese");
                    ui.separator();
                    ui.label("本工具用于一键切换游戏语言为中文，并安装字体修复 MOD。");
                    ui.label("MOD 版本: 5.1.0");
                    ui.separator();
                    ui.label("许可证 (MIT):");
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        ui.monospace(Self::license_text());
                    });
                    ui.separator();
                    if ui.button("关闭").clicked() {
                        self.show_about = false;
                    }
                });
        }
    }
}