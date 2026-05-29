# SFS 汉化

本项目提供 Spaceflight Simulator 的字体修复 MOD 与汉化工具 解决游戏中文字符显示为口的问题

## 原理

游戏的 normal 字体不含中文字形 MOD 在运行时将其替换为 Noto Sans SC 并创建 TextMeshPro 动态字体

## 项目结构

```
mod/             MOD 源码 (C# Harmony Patch)
tool/rust/      Rust 桌面安装工具
tool/python/    Python 桌面工具
legacy/         旧版本参考
```

## 参与

欢迎提交 Issue 和 Pull Request 帮助改进翻译 修复 Bug 或优化性能

项目地址 https://github.com/Dere3046/SFS_Chinese
翻译仓库 https://github.com/sTheNight/Spaceflight-Simulator-CNlang
