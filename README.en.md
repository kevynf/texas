<h1 align="center">
  <img src="icons/texas/texas.svg" width=200 height=200/><br>
  Texas
</h1>

<h4 align="center">Lightweight Code Editor</h4>

<p align="center">
  <a href="README.md">中文</a> | English
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

Texas is a lightweight code editor specialized from Lapce and
decoupled from any specific coding harness, built with Rust,
[Floem](https://github.com/lapce/floem), and its editor core components. It
provides file browsing, basic editing, syntax highlighting, Git integration, a
built-in terminal, and English and Simplified Chinese UI support.

For the vibe coding era, Texas will continue to evolve as a fast, lightweight
code editor decoupled from specific coding harnesses, with a focus on reading
code and reviewing changes.

## Features

* Syntax highlighting via Tree-sitter
* Modal editing support (Vim-like, toggleable)
* Built-in terminal
* Git integration for viewing and committing changes
* UI internationalization (English and Simplified Chinese)

## Building from source

Install Rust with [`rustup.rs`](https://rustup.rs/) (minimum version 1.87), then:

```sh
cargo build --frozen --bin texas
```

For platform-specific system dependencies, see [docs/building-from-source.en.md](docs/building-from-source.en.md).

Windows releases are currently distributed as a portable ZIP. Build, test, and
release details are documented in
[docs/building-from-source.en.md](docs/building-from-source.en.md). See
[docs/i18n.md](docs/i18n.md) for the internationalization implementation.

## Contributing

See [CONTRIBUTING.en.md](CONTRIBUTING.en.md) for guidelines.

## License

Texas is released under the Apache License Version 2.0. See [`LICENSE`](LICENSE)
for the license text and [`NOTICE`](NOTICE) for attribution notices.
