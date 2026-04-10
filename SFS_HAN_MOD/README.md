# SFS_HAN_MOD - SFS 字体修复 MOD

修复 Spaceflight Simulator 中文显示为 `口` 的问题。

## 功能

- 替换游戏默认 `normal` 字体为 Noto Sans SC
- 动态创建 TextMeshPro 字体（支持所有中文字符）
- 修复 HUD 上的时间加速、质量、推力等数值显示

## 依赖

| 依赖 | 说明 |
|------|------|
| **BepInEx 5.x** | 提供 Harmony 运行时（游戏已内置 `0Harmony.dll`） |
| **NotoSansSC.ttf** | 已包含在发布包中 |
| **游戏版本 1.5.9+** | 测试通过 |

> 游戏 `Managed/` 目录已包含 `0Harmony.dll`，MOD 通过它进行 Harmony Patch。

## 安装

1. 将 `SFS_HAN_MOD` 文件夹复制到游戏的 `Mods/` 目录下：

```
Spaceflight Simulator Game
└── Mods/
    └── SFS_HAN_MOD/
        ├── SFS_HAN_MOD.dll
        └── NotoSansSC.ttf
```

## 使用

启动游戏后，MOD 会自动加载并修复中文显示。在游戏内 MOD 菜单中可以启用/禁用此 MOD。

## 版本

**MOD 版本**: 5.1.0
**支持游戏版本**: 1.5.9+

## 编译

```bash
cd SFS_HAN_MOD/SFS_HAN_MOD
dotnet build SFS_HAN_MOD.csproj -c Release
```

输出: `bin/Release/SFS_HAN_MOD.dll`

### 依赖说明

`lib/` 目录应当包含所有编译所需的 DLL 引用 包括不限于如下（来自v1.6版本游戏的 `Managed/` 目录）：
- `0Harmony.dll`
- `Assembly-CSharp.dll`
- `Unity.TextMeshPro.dll`
- `UnityEngine*.dll`