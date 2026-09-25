<h1 align="center">
  <img src="icons/texas/texas.svg" width=200 height=200/><br>
  Texas
</h1>

<h4 align="center">轻量级代码编辑器</h4>

<p align="center">
  中文 | <a href="README.en.md">English</a>
</p>

<p align="center">
  <a href="https://github.com/kevynf/texas/actions/workflows/ci.yml">
    <img src="https://github.com/kevynf/texas/actions/workflows/ci.yml/badge.svg?branch=master" alt="CI" />
  </a>
  <a href="https://github.com/kevynf/texas/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="Apache-2.0 license" />
  </a>
</p>

<br/>

Texas 是一个基于 Lapce 特化开发的轻量级代码编辑器，采用 Rust、[Floem](https://github.com/lapce/floem) 及其编辑器核心组件构建，提供文件浏览、基础编辑、语法高亮、Git 集成、内置终端，以及英文和简体中文界面。

面向 vibe coding 时代，Texas 将持续迭代，致力于成为一个启动迅速、专注于代码阅读与变更审查的轻量级代码编辑器。Texas 不会内置 AI 功能，也不计划引入 AI 相关插件，而是作为独立工具与各类 coding harness 配合使用。

## 特性

* 基于 Tree-sitter 的语法高亮
* 模态编辑支持（类 Vim，可切换）
* 内置终端
* Git 集成，支持查看和提交变更
* 界面国际化（英文和简体中文）

## 从源码构建

使用 [`rustup.rs`](https://rustup.rs/) 安装 Rust（最低版本 1.87），然后：

```sh
cargo build --frozen --bin texas
```

各平台的系统依赖详见 [docs/building-from-source.md](docs/building-from-source.md)。

Windows 发布目前提供便携版 ZIP。构建、测试和发布说明见
[docs/building-from-source.md](docs/building-from-source.md)。国际化实现说明见
[docs/i18n.md](docs/i18n.md)。

## 贡献

请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

## 许可证

Texas 基于 Apache License Version 2.0 发布。许可证文本见 [`LICENSE`](LICENSE)，
上游代码和第三方资源归属见 [`NOTICE`](NOTICE)。
