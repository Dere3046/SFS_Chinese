# SFS 汉化

Spaceflight Simulator 字体修复 MOD + 语言工具。

## 原理

游戏的 `normal` 字体不含中文字形。本 MOD 在运行时将其替换为 Noto Sans SC，并创建 TextMeshPro 动态字体，
使游戏中所有中文文本正确显示（不再出现 `口`）。

## 目录结构

```
mod/                    # MOD 源码 (C#)
  SFS_HAN_MOD.cs        # 原生 ModLoader 版
  SFS_HAN_MOD.csproj
  FontFixPlugin.cs      # BepInEx 旧版兼容
  SFS_FontFix.csproj
  lib/                  # 引用 DLL（开发者手动从游戏目录复制）
  update_lib.sh         # 从游戏目录同步 DLL 的脚本

tool/
  rust/                 # Rust 桌面工具 (egui)
  python/               # Python 工具 (tkinter, 旧版)

legacy/
  SFS_Language_TOOL/    # 旧 Rust 工具归档
```

## 编译

### MOD

```bash
# 1. 从游戏目录复制依赖 DLL
./mod/update_lib.sh "C:/path/to/Spaceflight Simulator Game"

# 2. 编译
dotnet build mod/SFS_HAN_MOD.csproj -c Release    # 原生版 → dist/SFS_HAN_MOD.dll
dotnet build mod/SFS_FontFix.csproj -c Release    # BepInEx 版 → dist/SFS_FontFix.dll
```

### Rust 工具

```bash
cd tool/rust
cargo build --release
```

## 安装

### 原生 MOD（推荐）

将 `SFS_HAN_MOD.dll` + `NotoSansSC.ttf` 放入 `Mods/SFS_HAN_MOD/`：
```
Spaceflight Simulator/
└── Mods/
    └── SFS_HAN_MOD/
        ├── SFS_HAN_MOD.dll
        └── NotoSansSC.ttf
```

### BepInEx 版（旧用户兼容）

将 `SFS_FontFix.dll` + `NotoSansSC.ttf` 放入 `BepInEx/plugins/SFS_FontFix/`。

## 技术方案

3 个轻量 Harmony Patch，不拦截 per-frame 更新：

| Hook | 时机 | 开销 |
|------|------|------|
| `TranslationManager.Awake` Postfix | 启动时 1 次 | 替换 fonts[0]，全量扫描 TMP |
| `TMP_Text.OnEnable` Postfix | 组件激活时 | O(1) 引用比较 + 赋值 |
| `TranslationManager.SetLanguage` Postfix | 切语言时 | O(1) |

## 引用

- BepInEx v5.4.23.5 ([LGPL-2.1](https://github.com/BepInEx/BepInEx/blob/master/LICENSE))
- Noto Sans SC ([SIL Open Font License](https://fonts.google.com/specimen/Noto+Sans+SC/about))
