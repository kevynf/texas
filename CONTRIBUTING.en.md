# How to contribute

[中文](CONTRIBUTING.md) | English

---

Thank you for your interest in contributing to Texas! No contribution is too small and we consider _all_ contributions to the project.

## Feature Requests

A feature request is _editor behaviour that you want to have included in Texas_. We track feature requests on GitHub via [issues](../../issues). There are generally two kinds of features:

### Core features

A feature more suited to the core development of Texas. If this is the case please make a suggestion in an [issue](../../issues).

### Syntax highlighting

Syntax highlighting is provided via Tree-sitter grammars.

---

To reduce the number of duplicate requests, please search through the issues to see if something has already been suggested. If a feature you want has been suggested, then comment on that issue to let us know it's popular. You can use emoji-reactions to show us just how popular.

## Bug Reports

Bugs should also be reported on GitHub via [issues](../../issues). This allows us to track them and see how prevalent they are.

If you encounter a bug when using Texas, check the issues to see if anyone else has encountered it. If it already exists, you can use emoji reactions so we can see community interest in specific issues and how important they are.

## Pull Requests

If you want to write some code, develop the documentation, or otherwise work on a certain feature or bug, let us know by replying to or creating an [issue](../../issues).

Before submitting changes, run the same checks as CI:

```sh
cargo fmt --all --check
cargo clippy --workspace
cargo build --frozen --bin texas
cargo test --workspace
```

These checks keep formatting, linting, compilation, and tests local and predictable.

Pull requests should use the repository template and include a concise summary, scope and non-goals, the validation commands that were actually run, and any known risks or follow-up work. LLM-assisted changes follow the same requirements and must not claim checks or review results that have not occurred.

When changing documentation, keep the Chinese and English versions in sync.
Keep project documentation short and focus on the README, build instructions,
and technical notes needed to maintain the current implementation.
