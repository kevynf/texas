# 从源码构建

中文 | [English](building-from-source.en.md)

---

Cargo 负责构建流程。使用 [`rustup.rs`](https://rustup.rs/) 安装 Rust，然后安装操作系统所需的系统依赖。

### Linux 依赖

#### Ubuntu

```sh
sudo apt install clang libxkbcommon-x11-dev pkg-config libvulkan-dev libwayland-dev xorg-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

#### Fedora

```sh
sudo dnf install clang libxkbcommon-devel libxcb-devel vulkan-loader-devel wayland-devel openssl-devel pkgconf
```

#### Void Linux

```sh
sudo xbps-install -S base-devel clang libxkbcommon-devel vulkan-loader wayland-devel
```

### 构建 Texas

克隆仓库并进入目录：

```sh
git clone <repo-url> ~/texas
cd ~/texas
```

构建并安装应用：

```sh
cargo install --path . --bin texas --profile release-lto --locked
```

在 Windows 上无需额外的构建步骤；自动发布工作流会打包主 `texas` 可执行文件的便携 ZIP。

开发构建可以使用：

```sh
cargo build --frozen --bin texas
cargo run --profile fastdev --bin texas
```

Windows 便携版使用以下命令构建：

```powershell
cargo build --frozen --profile release-lto --features texas-app/portable --bin texas
```

生成的可执行文件位于 `target/release-lto/texas.exe`。GitHub Actions 的发布
工作流会将该文件与固定版本的语法 grammar 和 query 一起打包为
`Texas-windows-portable.zip`。

常用检查命令：

```sh
cargo fmt --all --check
cargo clippy --workspace
cargo test --workspace
```

Texas 编译完成后，可执行文件位于 `$HOME/.cargo/bin/texas`，应自动加入 `PATH`。
