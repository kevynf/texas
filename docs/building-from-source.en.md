# Building from source

[中文](building-from-source.md) | English

---

Cargo handles the build process. Install Rust with [`rustup.rs`](https://rustup.rs/), then install the system dependencies required by your operating system.

### Linux dependencies

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

### Build Texas

Clone this repository and enter its directory:

```sh
git clone <repo-url> ~/texas
cd ~/texas
```

Build and install the application:

```sh
cargo install --path . --bin texas --profile release-lto --locked
```

For a development build, use:

```sh
cargo build --frozen --bin texas
cargo run --profile fastdev --bin texas
```

Build the Windows portable version with:

```powershell
cargo build --frozen --profile release-lto --features texas-app/portable --bin texas
```

The executable is written to `target/release-lto/texas.exe`. The GitHub Actions
release workflow packages it as `Texas-windows-portable.zip`.

Common checks are:

```sh
cargo fmt --all --check
cargo clippy --workspace
cargo test --workspace
```

Once Texas is compiled, the executable will be available in `$HOME/.cargo/bin/texas` and should be available in `PATH` automatically.
