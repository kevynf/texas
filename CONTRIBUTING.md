# 贡献指南

中文 | [English](CONTRIBUTING.en.md)

---

感谢你考虑为 Texas 做出贡献！任何大小的贡献我们都欢迎。

## 功能请求

功能请求是指_你希望 Texas 包含的编辑器行为_。我们通过 GitHub [issues](../../issues) 追踪功能请求。功能请求一般分为两类：

### 核心功能

适合 Texas 核心开发的功能。请在 [issue](../../issues) 中提出建议。

### 语法高亮

语法高亮通过 Tree-sitter 语法文件提供。

---

为减少重复请求，请先搜索现有 issues。如果已有类似建议，可以通过评论或表情反应来表达支持。

## Bug 报告

Bug 同样通过 GitHub [issues](../../issues) 报告。

如果你在使用 Texas 时遇到 bug，请先查看是否已有人报告。如果已有，可以通过表情反应来表示该问题的重要性。

## 拉取请求

如果你想编写代码、开发文档或修复某个功能或 bug，请先通过 [issue](../../issues) 告知我们。

提交变更前，请运行与 CI 相同的检查：

```sh
cargo fmt --all --check
cargo clippy --workspace
cargo build --frozen --bin texas
cargo test --workspace
```

这些检查确保格式、静态分析、编译和测试在本地一致且可预测。

拉取请求应使用仓库模板，包含简要摘要、范围和非目标、实际运行的验证命令，以及已知风险或后续工作。LLM 辅助的变更遵循相同要求，不得声称未实际执行的检查结果。

修改文档时，请同步更新对应的中英文文件。项目文档保持简短，优先维护
README、构建说明和当前实现所需的技术说明。
