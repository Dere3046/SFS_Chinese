# SFS 字体修复 MOD

BepInEx 插件，替换游戏默认字体解决中文显示 `口` 的问题。

## 声明

- **技术栈说明**：本部分内容/相关操作并非本人主要技术领域，可能存在不完善之处。
- **BepInEx 集成与许可**：
    - 本项目已包含 BepInEx 的发布文件。     
    - 文件来源：[BepInEx v5.4.23.5](https://github.com/BepInEx/BepInEx/releases/tag/v5.4.23.5)
    - BepInEx 采用 **LGPL-2.1** 许可证，详情请参见：[BepInEx/LICENSE](https://github.com/BepInEx/BepInEx/blob/master/LICENSE)

## 原理

游戏的 `normal` 字体不含中文字形。此 MOD 将其替换为 Noto Sans SC，并创建 TextMeshPro 动态字体。

## 使用

将 `SFS_FontFix.dll` 和 `NotoSansSC.ttf` 放入 `BepInEx/plugins/SFS_FontFix/`。

## SFS I'M SB

我没招了 我只能说SFS的PC版本就是没想过多语言化 甚至资源直接挪用了PE 且一些设置与其他内容是没有在翻译文件中有对照键的(后续会写中文补齐补丁) ;做游戏的时候至少考虑一下多语言支持吧