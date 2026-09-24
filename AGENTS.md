# Repository Guidelines

## Project Structure & Module Organization

Texas is a Rust 2024 workspace. `texas-app/` contains the Floem UI and the
`texas` binary entry point; `texas-core/` owns editor primitives, syntax, and
encoding; `texas-proxy/` handles filesystem, terminal, and dispatch work; and
`texas-rpc/` defines messages shared across those boundaries. Default settings
and themes live in `defaults/`, application translations in
`texas-app/assets/locales/`, icons in `icons/`, schemas in `extra/schemas/`, and
maintainer documentation in `docs/`. Tests normally sit beside the Rust module
inside `#[cfg(test)] mod tests`; benchmarks are under `texas-app/benches/`.

## Build, Test, and Development Commands

Run commands from the repository root with the stable toolchain (Rust 1.87 or
newer):

```sh
cargo run --profile fastdev --bin texas      # run a responsive development build
cargo build --frozen --bin texas             # build with the committed lockfile
cargo fmt --all --check                      # verify rustfmt (85-column limit)
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                       # run every crate's tests
```

Use focused commands while iterating, such as `cargo test -p texas-core --lib`.
Platform prerequisites and release builds are documented in
`docs/building-from-source.en.md`.

## Coding Style & Naming Conventions

Accept `rustfmt` output; use four-space indentation and do not align code by
hand. Follow Rust naming conventions: `snake_case` for modules and functions,
`UpperCamelCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants.
Keep crate boundaries intact and reuse workspace dependencies from the root
`Cargo.toml`. Format TOML according to `.taplo.toml`. When changing UI text,
update both `en.toml` and `zh-CN.toml` with matching keys and placeholders.

## Testing Guidelines

Add unit tests near the behavior they cover and give tests descriptive
`snake_case` names. Run the affected crate first, then `cargo test --workspace`.
Localization and configuration changes must retain the consistency checks in
`texas-app/src/i18n.rs`. No numeric coverage threshold is configured; prioritize
regression tests for parsing, state transitions, and cross-crate messages.

## Commit & Pull Request Guidelines

Recent history uses Conventional Commit subjects, commonly `feat:`, `fix:`,
`refactor:`, `style:`, `build:`, and `chore:`, followed by concise Chinese text.
Keep each commit to one logical change. Pull requests should follow
`.github/PULL_REQUEST_TEMPLATE.md`: summarize the change, state scope and
non-goals, list only validation actually run, and note risks or follow-up work.
Link relevant issues and include screenshots for visible UI changes. Keep paired
Chinese and English documentation synchronized.
