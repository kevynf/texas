# Repository Guidelines

## Build and verification

- Run Cargo commands from the repository root. `rust-toolchain.toml` tracks **stable**; Rust 1.87 is the manifest minimum, not a pinned toolchain.
- Fetch before a frozen build on a fresh checkout. Floem/editor-core and several other dependencies are Git-revision-pinned; preserve those pins and `Cargo.lock` unless changing dependencies intentionally.
- CI runs on Windows. Match `.github/workflows/ci.yml` (its Clippy flags are stricter than the build docs):

```sh
cargo fetch --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --frozen --bin texas
cargo test --workspace
```

- Use `--workspace` for full checks: plain `cargo test` at this root selects the root package, not all library tests.
- Development: `cargo run --profile fastdev --bin texas`. `fastdev` optimizes dependencies while keeping workspace code in dev mode.
- Focused tests: `cargo test -p texas-core --lib`; single test: `cargo test -p texas-core --lib lens::tests::test_lens_metric -- --exact`.
- UI translation checks: `cargo test -p texas-app --lib i18n::tests`. Visual-line performance: `cargo bench -p texas-app --bench visual_line`.
- Rustfmt uses an 85-column limit; manifest TOML formatting follows `.taplo.toml`. Linux system packages are listed in `docs/building-from-source.en.md`.

## Wiring and build gotchas

- The workspace has the root `texas` package plus four library packages. The root binary points to `texas-app/src/bin/texas.rs`, which calls `texas_app::app::launch()`; CLI parsing and startup live in `texas-app/src/app.rs`.
- Cargo also auto-discovers that binary under `texas-app`. Build the **root** package for the shipped executable (`cargo build --frozen --bin texas`, or explicitly `-p texas`): root `build.rs` embeds the Windows icon generated from `icons/texas/texas.svg`.
- `texas-app/src/proxy.rs` starts the local `texas-proxy::dispatch::Dispatcher` and RPC handlers on threads in the UI process. Follow `texas-rpc` request/notification types when tracing file, Git, watcher, or terminal operations.
- `texas-core/src/lib.rs` re-exports `floem_editor_core::*`; many editor primitives used through `texas_core` are implemented in the pinned upstream dependency. Local core code handles syntax, language metadata, encoding, and application directories.
- `texas-core/build.rs` generates `OUT_DIR/meta.rs`. A `RELEASE_TAG_NAME` beginning with `v` selects Stable; otherwise builds use Debug/Nightly metadata. This also affects application data directory names via `texas-core/src/directory.rs`.
- Windows MSVC builds statically link the CRT via `.cargo/config.toml`. Portable builds use `cargo build --frozen --profile release-lto --features texas-app/portable --bin texas`; their data lives under `texas-data` beside the executable.
- Keep settings structs in `texas-app/src/config/` aligned with embedded `defaults/settings.toml`: missing or misnamed required fields can panic during default configuration loading.
- Keep the apparently unused `cosmic-text` dependency in `texas-app/Cargo.toml`: it enables `monospace_fallback` for Floem's shared text stack and CJK rendering.

## UI text and documentation

- Use `CommonData.i18n`, not Floem's runtime-global context: workspace tabs can have different language settings. Use reactive producers such as `label(i18n.text_signal("key"))` to update text on language changes.
- Add UI keys to both `texas-app/assets/locales/en.toml` and `zh-CN.toml`, with matching placeholders. Command and settings changes also require locale entries; `texas-app/src/i18n.rs` tests enforce coverage.
- Treat `texas-app/src/i18n.rs` as authoritative when consulting `docs/i18n.md`: proxy messages are currently plain strings, and `text_with_args` takes an explicit fallback argument. The literal-key test only scans `text` and `text_signal` calls.
- Keep paired Chinese/English READMEs, contribution guides, and build docs in sync (`CONTRIBUTING.en.md`).

## Contribution constraints

- Inspect status and the full diff before editing or submitting; preserve unrelated work. Do not push, create/merge PRs, or delete remote branches without explicit authorization.
- Use Conventional Commit-style titles and `.github/PULL_REQUEST_TEMPLATE.md` (scope/non-goals, actual validation, risks/follow-ups). Distinguish local checks from CI results; report only checks actually run.
