#[derive(Debug, Clone)]
pub enum AppLanguage {
    Chinese,
    English,
}

impl AppLanguage {
    pub fn display_name(&self) -> &str {
        match self {
            AppLanguage::Chinese => "中文",
            AppLanguage::English => "English",
        }
    }
    
    pub fn app_title(&self) -> &str {
        match self {
            AppLanguage::Chinese => "SFS 语言工具",
            AppLanguage::English => "SFS Language Tool",
        }
    }
    
    pub fn language_label(&self) -> &str {
        match self {
            AppLanguage::Chinese => "界面语言",
            AppLanguage::English => "UI Language",
        }
    }
    
    pub fn section_game_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "游戏目录",
            AppLanguage::English => "Game Directory",
        }
    }
    
    pub fn label_game_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "游戏目录:",
            AppLanguage::English => "Game Dir:",
        }
    }
    
    pub fn label_no_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未选择",
            AppLanguage::English => "Not selected",
        }
    }
    
    pub fn btn_select_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "选择游戏目录",
            AppLanguage::English => "Select Game Directory",
        }
    }
    
    pub fn btn_auto_find(&self) -> &str {
        match self {
            AppLanguage::Chinese => "自动寻找",
            AppLanguage::English => "Auto Find",
        }
    }
    
    pub fn label_found_dirs(&self) -> &str {
        match self {
            AppLanguage::Chinese => "找到的游戏目录:",
            AppLanguage::English => "Found game directories:",
        }
    }
    
    pub fn btn_clear_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "清除结果",
            AppLanguage::English => "Clear Results",
        }
    }
    
    pub fn error_no_game_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未找到游戏目录",
            AppLanguage::English => "No game directory found",
        }
    }
    
    pub fn section_custom_files(&self) -> &str {
        match self {
            AppLanguage::Chinese => "自定义翻译文件",
            AppLanguage::English => "Custom Translation Files",
        }
    }
    
    pub fn desc_custom_files(&self) -> &str {
        match self {
            AppLanguage::Chinese => "管理自定义翻译文件 (位于 Spaceflight Simulator_Data/Custom Translations/ 目录)",
            AppLanguage::English => "Manage custom translation files (in Spaceflight Simulator_Data/Custom Translations/ directory)",
        }
    }
    
    pub fn label_existing_files(&self) -> &str {
        match self {
            AppLanguage::Chinese => "现有文件:",
            AppLanguage::English => "Existing files:",
        }
    }
    
    pub fn label_no_files(&self) -> &str {
        match self {
            AppLanguage::Chinese => "暂无文件",
            AppLanguage::English => "No files yet",
        }
    }
    
    pub fn label_new_filename(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件名:",
            AppLanguage::English => "Filename:",
        }
    }
    
    pub fn btn_create(&self) -> &str {
        match self {
            AppLanguage::Chinese => "创建",
            AppLanguage::English => "Create",
        }
    }
    
    pub fn btn_edit(&self) -> &str {
        match self {
            AppLanguage::Chinese => "编辑",
            AppLanguage::English => "Edit",
        }
    }
    
    pub fn btn_backup(&self) -> &str {
        match self {
            AppLanguage::Chinese => "备份",
            AppLanguage::English => "Backup",
        }
    }
    
    pub fn btn_restore(&self) -> &str {
        match self {
            AppLanguage::Chinese => "恢复",
            AppLanguage::English => "Restore",
        }
    }
    
    pub fn btn_delete(&self) -> &str {
        match self {
            AppLanguage::Chinese => "删除",
            AppLanguage::English => "Delete",
        }
    }
    
    pub fn btn_compare_keys(&self) -> &str {
        match self {
            AppLanguage::Chinese => "对比翻译键",
            AppLanguage::English => "Compare Keys",
        }
    }
    
    pub fn error_no_file_selected(&self) -> &str {
        match self {
            AppLanguage::Chinese => "请先选择文件",
            AppLanguage::English => "Please select a file first",
        }
    }
    
    pub fn label_no_fonts_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未找到字体",
            AppLanguage::English => "No fonts found",
        }
    }
    
    pub fn label_select_font(&self) -> &str {
        match self {
            AppLanguage::Chinese => "选择字体",
            AppLanguage::English => "Select Font",
        }
    }
    
    pub fn section_lang_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "语言设置 (LanguageSettings_2.txt)",
            AppLanguage::English => "Language Settings (LanguageSettings_2.txt)",
        }
    }
    
    pub fn desc_lang_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "编辑游戏的语言设置文件",
            AppLanguage::English => "Edit game language settings file",
        }
    }
    
    pub fn btn_backup_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "备份设置文件",
            AppLanguage::English => "Backup Settings",
        }
    }
    
    pub fn btn_restore_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "恢复设置文件",
            AppLanguage::English => "Restore Settings",
        }
    }
    
    pub fn checkbox_use_custom(&self) -> &str {
        match self {
            AppLanguage::Chinese => "使用自定义翻译",
            AppLanguage::English => "Use Custom Translation",
        }
    }
    
    pub fn label_select_custom(&self) -> &str {
        match self {
            AppLanguage::Chinese => "选择自定义文件:",
            AppLanguage::English => "Select custom file:",
        }
    }
    
    pub fn label_select_official(&self) -> &str {
        match self {
            AppLanguage::Chinese => "选择官方语言:",
            AppLanguage::English => "Select official language:",
        }
    }
    
    pub fn label_select_file(&self) -> &str {
        match self {
            AppLanguage::Chinese => "选择文件",
            AppLanguage::English => "Select file",
        }
    }
    
    pub fn label_language(&self) -> &str {
        match self {
            AppLanguage::Chinese => "语言",
            AppLanguage::English => "Language",
        }
    }
    
    pub fn btn_save_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "保存设置",
            AppLanguage::English => "Save Settings",
        }
    }
    
    pub fn label_settings_preview(&self) -> &str {
        match self {
            AppLanguage::Chinese => "设置预览:",
            AppLanguage::English => "Settings Preview:",
        }
    }
    
    pub fn status_dir_loaded(&self) -> &str {
        match self {
            AppLanguage::Chinese => "游戏目录已加载",
            AppLanguage::English => "Game directory loaded",
        }
    }
    
    pub fn status_file_created(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件已创建",
            AppLanguage::English => "File created",
        }
    }
    
    pub fn status_file_deleted(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件已删除",
            AppLanguage::English => "File deleted",
        }
    }
    
    pub fn status_settings_saved(&self) -> &str {
        match self {
            AppLanguage::Chinese => "设置已保存",
            AppLanguage::English => "Settings saved",
        }
    }
    
    pub fn status_backup_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "备份成功",
            AppLanguage::English => "Backup successful",
        }
    }
    
    pub fn status_restore_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "恢复成功",
            AppLanguage::English => "Restore successful",
        }
    }
    
    pub fn status_download_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "下载成功",
            AppLanguage::English => "Download successful",
        }
    }
    
    pub fn error_invalid_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "无效的游戏目录",
            AppLanguage::English => "Invalid game directory",
        }
    }
    
    pub fn error_empty_filename(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件名不能为空",
            AppLanguage::English => "Filename cannot be empty",
        }
    }
    
    pub fn error_no_txt_extension(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件必须以 .txt 结尾",
            AppLanguage::English => "File must end with .txt",
        }
    }
    
    pub fn error_file_exists(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件已存在",
            AppLanguage::English => "File already exists",
        }
    }
    
    pub fn error_create_file(&self) -> &str {
        match self {
            AppLanguage::Chinese => "创建文件失败",
            AppLanguage::English => "Failed to create file",
        }
    }
    
    pub fn error_no_dir(&self) -> &str {
        match self {
            AppLanguage::Chinese => "请先选择游戏目录",
            AppLanguage::English => "Please select game directory first",
        }
    }
    
    pub fn error_save_settings(&self) -> &str {
        match self {
            AppLanguage::Chinese => "保存设置失败",
            AppLanguage::English => "Failed to save settings",
        }
    }
    
    pub fn error_backup_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "备份失败",
            AppLanguage::English => "Backup failed",
        }
    }
    
    pub fn error_restore_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "恢复失败",
            AppLanguage::English => "Restore failed",
        }
    }
    
    pub fn error_empty_url(&self) -> &str {
        match self {
            AppLanguage::Chinese => "URL不能为空",
            AppLanguage::English => "URL cannot be empty",
        }
    }
    
    pub fn error_invalid_filename(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件名无效，必须以.txt结尾",
            AppLanguage::English => "Invalid filename, must end with .txt",
        }
    }
    
    pub fn error_download_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "下载失败",
            AppLanguage::English => "Download failed",
        }
    }
    
    pub fn label_download_from_url(&self) -> &str {
        match self {
            AppLanguage::Chinese => "从网络下载翻译文件",
            AppLanguage::English => "Download translation from network",
        }
    }
    
    pub fn label_url(&self) -> &str {
        match self {
            AppLanguage::Chinese => "URL:",
            AppLanguage::English => "URL:",
        }
    }
    
    pub fn label_filename(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件名:",
            AppLanguage::English => "Filename:",
        }
    }
    
    pub fn btn_download(&self) -> &str {
        match self {
            AppLanguage::Chinese => "下载",
            AppLanguage::English => "Download",
        }
    }
    
    pub fn btn_about(&self) -> &str {
        match self {
            AppLanguage::Chinese => "关于",
            AppLanguage::English => "About",
        }
    }
    
    pub fn btn_close(&self) -> &str {
        match self {
            AppLanguage::Chinese => "关闭",
            AppLanguage::English => "Close",
        }
    }
    
    pub fn about_title(&self) -> &str {
        match self {
            AppLanguage::Chinese => "关于 SFS 语言工具",
            AppLanguage::English => "About SFS Language Tool",
        }
    }
    
    pub fn about_developer(&self) -> &str {
        match self {
            AppLanguage::Chinese => "开发者: Dere3046",
            AppLanguage::English => "Developer: Dere3046",
        }
    }
    
    pub fn about_project(&self) -> &str {
        match self {
            AppLanguage::Chinese => "项目地址: https://github.com/Dere3046/SFS_LANG_TOOL",
            AppLanguage::English => "Project: https://github.com/Dere3046/SFS_LANG_TOOL",
        }
    }
    
    pub fn about_license_title(&self) -> &str {
        match self {
            AppLanguage::Chinese => "许可证: MIT",
            AppLanguage::English => "License: MIT",
        }
    }
    
    pub fn about_license_content(&self) -> &str {
        "MIT License\n\nCopyright (c) 2026 Dere3046\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."
    }
    
    pub fn compare_title(&self) -> &str {
        match self {
            AppLanguage::Chinese => "翻译键对比",
            AppLanguage::English => "Translation Keys Comparison",
        }
    }
    
    pub fn compare_missing(&self) -> &str {
        match self {
            AppLanguage::Chinese => "缺少的键 (在Example.txt中存在但文件中不存在):",
            AppLanguage::English => "Missing keys (in Example.txt but not in file):",
        }
    }
    
    pub fn compare_extra(&self) -> &str {
        match self {
            AppLanguage::Chinese => "额外的键 (在文件中存在但Example.txt中不存在):",
            AppLanguage::English => "Extra keys (in file but not in Example.txt):",
        }
    }
    
    pub fn compare_none(&self) -> &str {
        match self {
            AppLanguage::Chinese => "无",
            AppLanguage::English => "None",
        }
    }
    
    pub fn section_backup_manager(&self) -> &str {
        match self {
            AppLanguage::Chinese => "备份管理",
            AppLanguage::English => "Backup Manager",
        }
    }
    
    pub fn desc_backup_manager(&self) -> &str {
        match self {
            AppLanguage::Chinese => "管理和恢复备份文件",
            AppLanguage::English => "Manage and restore backup files",
        }
    }
    
    pub fn btn_refresh_backups(&self) -> &str {
        match self {
            AppLanguage::Chinese => "刷新备份列表",
            AppLanguage::English => "Refresh Backups",
        }
    }
    
    pub fn label_custom_backups(&self) -> &str {
        match self {
            AppLanguage::Chinese => "自定义翻译备份:",
            AppLanguage::English => "Custom Translation Backups:",
        }
    }
    
    pub fn label_settings_backups(&self) -> &str {
        match self {
            AppLanguage::Chinese => "语言设置备份:",
            AppLanguage::English => "Language Settings Backups:",
        }
    }
    
    pub fn label_no_backups(&self) -> &str {
        match self {
            AppLanguage::Chinese => "暂无备份",
            AppLanguage::English => "No backups found",
        }
    }
    
    pub fn btn_rename(&self) -> &str {
        match self {
            AppLanguage::Chinese => "重命名",
            AppLanguage::English => "Rename",
        }
    }
    
    pub fn label_rename_backup(&self) -> &str {
        match self {
            AppLanguage::Chinese => "重命名备份文件:",
            AppLanguage::English => "Rename backup file:",
        }
    }
    
    pub fn btn_confirm(&self) -> &str {
        match self {
            AppLanguage::Chinese => "确认",
            AppLanguage::English => "Confirm",
        }
    }
    
    pub fn btn_cancel(&self) -> &str {
        match self {
            AppLanguage::Chinese => "取消",
            AppLanguage::English => "Cancel",
        }
    }
    
    pub fn status_rename_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "重命名成功",
            AppLanguage::English => "Rename successful",
        }
    }
    
    pub fn error_rename_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "重命名失败",
            AppLanguage::English => "Rename failed",
        }
    }
    
    pub fn error_invalid_bak_name(&self) -> &str {
        match self {
            AppLanguage::Chinese => "文件名必须以.bak结尾",
            AppLanguage::English => "Filename must end with .bak",
        }
    }
    
    pub fn error_no_backup_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未找到备份文件",
            AppLanguage::English => "No backup file found",
        }
    }
    
    pub fn label_font_config(&self) -> &str {
        match self {
            AppLanguage::Chinese => "字体配置 (写入翻译文件):",
            AppLanguage::English => "Font Configuration (write to translation file):",
        }
    }
    
    pub fn btn_apply_font(&self) -> &str {
        match self {
            AppLanguage::Chinese => "应用字体",
            AppLanguage::English => "Apply Font",
        }
    }
    
    pub fn status_font_applied(&self) -> &str {
        match self {
            AppLanguage::Chinese => "字体已应用到文件",
            AppLanguage::English => "Font applied to file",
        }
    }
    
    pub fn error_font_apply_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "应用字体失败",
            AppLanguage::English => "Failed to apply font",
        }
    }
    
    pub fn error_no_font_selected(&self) -> &str {
        match self {
            AppLanguage::Chinese => "请先选择字体",
            AppLanguage::English => "Please select a font first",
        }
    }
    
    pub fn font_config_warning(&self) -> &str {
        match self {
            AppLanguage::Chinese => "注意：游戏只使用一个字体\"normal\"。所有语言共享同一字体，如果字体不支持某些字符会显示\"口\"。这是游戏本身的限制，无法通过工具修改。",
            AppLanguage::English => "Note: The game only uses one font \"normal\". All languages share the same font. If the font doesn't support certain characters, \"口\" will be displayed. This is a game limitation that cannot be changed by this tool.",
        }
    }
    
    pub fn section_bepinex(&self) -> &str {
        match self {
            AppLanguage::Chinese => "BepInEx 字体修复",
            AppLanguage::English => "BepInEx Font Fix",
        }
    }
    
    pub fn desc_bepinex(&self) -> &str {
        match self {
            AppLanguage::Chinese => "安装BepInEx框架和字体修复插件，解决游戏中文显示\"口\"的问题",
            AppLanguage::English => "Install BepInEx framework and font fix plugin to resolve Chinese character display issues",
        }
    }
    
    pub fn status_bepinex_installed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "✓ BepInEx 已安装 (v5.4.23.5)",
            AppLanguage::English => "✓ BepInEx Installed (v5.4.23.5)",
        }
    }

    pub fn status_bepinex_install_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "BepInEx 安装完成，包含字体修复插件",
            AppLanguage::English => "BepInEx installed successfully with font fix plugin",
        }
    }

    pub fn status_bepinex_not_installed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "✗ BepInEx 未安装",
            AppLanguage::English => "✗ BepInEx Not Installed",
        }
    }

    pub fn status_bepinex_uninstall_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "BepInEx 已完全卸载（包括所有插件和配置文件）",
            AppLanguage::English => "BepInEx completely uninstalled (including all plugins and configs)",
        }
    }
    
    pub fn status_font_fix_installed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "✓ 字体修复已安装",
            AppLanguage::English => "✓ Font Fix Installed",
        }
    }
    
    pub fn status_font_fix_not_installed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "✗ 字体修复未安装",
            AppLanguage::English => "✗ Font Fix Not Installed",
        }
    }
    
    pub fn btn_install_bepinex(&self) -> &str {
        match self {
            AppLanguage::Chinese => "安装 BepInEx",
            AppLanguage::English => "Install BepInEx",
        }
    }
    
    pub fn btn_install_font_fix(&self) -> &str {
        match self {
            AppLanguage::Chinese => "安装字体修复",
            AppLanguage::English => "Install Font Fix",
        }
    }
    
    pub fn status_font_fix_installed_msg(&self) -> &str {
        match self {
            AppLanguage::Chinese => "字体修复插件安装成功！重启游戏后生效",
            AppLanguage::English => "Font fix plugin installed! Restart game to apply",
        }
    }
    
    pub fn error_install_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "安装失败",
            AppLanguage::English => "Installation failed",
        }
    }
    
    pub fn error_bepinex_not_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未找到BepInEx文件，请确保BepInEx_win_x64_5.4.23.5文件夹在工具目录下",
            AppLanguage::English => "BepInEx files not found. Ensure BepInEx_win_x64_5.4.23.5 folder is in tool directory",
        }
    }
    
    pub fn label_current_version(&self) -> &str {
        match self {
            AppLanguage::Chinese => "当前版本",
            AppLanguage::English => "Current Version",
        }
    }
    
    pub fn btn_update_font_fix(&self) -> &str {
        match self {
            AppLanguage::Chinese => "更新字体修复",
            AppLanguage::English => "Update Font Fix",
        }
    }
    
    pub fn status_font_fix_updated(&self) -> &str {
        match self {
            AppLanguage::Chinese => "字体修复已强制更新到最新版本",
            AppLanguage::English => "Font fix force updated to latest version",
        }
    }
    
    pub fn error_plugin_files_not_found(&self) -> &str {
        match self {
            AppLanguage::Chinese => "未找到插件文件，请确保SFS_FontFix.dll和NotoSansSC.ttf在工具目录下",
            AppLanguage::English => "Plugin files not found. Ensure SFS_FontFix.dll and NotoSansSC.ttf are in tool directory",
        }
    }

    pub fn btn_uninstall_bepinex(&self) -> &str {
        match self {
            AppLanguage::Chinese => "卸载 BepInEx",
            AppLanguage::English => "Uninstall BepInEx",
        }
    }

    pub fn btn_uninstall_font_fix(&self) -> &str {
        match self {
            AppLanguage::Chinese => "卸载补丁",
            AppLanguage::English => "Uninstall Patch",
        }
    }

    pub fn status_uninstall_success(&self) -> &str {
        match self {
            AppLanguage::Chinese => "卸载成功",
            AppLanguage::English => "Uninstallation successful",
        }
    }

    pub fn error_uninstall_failed(&self) -> &str {
        match self {
            AppLanguage::Chinese => "卸载失败",
            AppLanguage::English => "Uninstallation failed",
        }
    }
}
