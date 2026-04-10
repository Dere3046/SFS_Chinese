# SFS Language Tool

Rust (egui) 桌面工具，管理 SFS 自定义翻译文件、BepInEx 与字体修复 MOD。

## 声明

- **技术栈说明**：本部分内容/相关操作并非本人主要技术领域，可能存在不完善之处。
- **BepInEx 集成与许可**：
    - 本项目已包含 BepInEx 的发布文件。     
    - 文件来源：[BepInEx v5.4.23.5](https://github.com/BepInEx/BepInEx/releases/tag/v5.4.23.5)
    - BepInEx 采用 **LGPL-2.1** 许可证，详情请参见：[BepInEx/LICENSE](https://github.com/BepInEx/BepInEx/blob/master/LICENSE)

## 功能

- 翻译文件 CRUD、备份、键值对比
- 语言设置管理 (LanguageSettings_2.txt)
- BepInEx 安装/卸载（完整清理残留文件）
- 字体修复 MOD 安装/更新/卸载

## 编译

```bash
cargo build --release
```

`assets/` 目录必须包含：
- `SFS_FontFix.dll` + `NotoSansSC.ttf`（编译时嵌入）
- `fonts/msyh.ttc` + `fonts/arial.ttf`（编译时嵌入）
- `BepInEx_win_x64_5.4.23.5/`（编译时复制到输出目录）

编译完成后，exe 同级目录会生成 `assets/`（GUI 字体用）和 `BepInEx_win_x64_5.4.23.5/`（安装用）。

## 吐槽

我没招了 我只能说SFS的PC版本就是没想过多语言化 甚至资源直接挪用了PE 且一些设置与其他内容是没有在翻译文件中有对照键的(后续会写中文补齐补丁)