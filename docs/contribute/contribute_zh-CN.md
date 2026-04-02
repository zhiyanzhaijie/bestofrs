# 贡献指南

如果你想帮助 `Best Of RS` 变得更好，有三种方式：

1. Recommend One（分享你对 Rust 生态的了解）
2. 提交 Bug 和功能需求（分享你的使用体验）
3. 贡献代码（通过 Pull Request 扩展或改进功能）

> 前两种方式在 GitHub 页面中都很容易找到入口。
> 本指南重点介绍如何为 `Best Of RS` 贡献代码。

## 前置知识
开始贡献前，请先阅读 [Architecture架构文档](../architecture/architecture_zh-CN.md)。

## 本地运行 Best Of RS

拉取main分支源码后，请先确保依赖为最新状态。
你需要安装与 `crates/ui/Cargo.toml` 中 Dioxus 版本兼容的 `dioxus-cli`。

开始开发前，请确认以下前置条件：

1. 准备好用于管理功能的 `GitHub Token`。
2. 启动你的 `Database`（推荐 Postgres；SQLite 在初始化后未经过充分测试, 编写指南这会它还是异常的）和 `Redis`。本地开发强烈建议使用 Docker。
3. 基于示例模板在 `crates/infra/src/config/` 下创建 `development.toml`，并填入实际配置项。

然后，在工作区根目录执行：

```bash
dx serve -p ui
```

你也可以使用实验性的热重载功能：

```bash
dx serve --hot-reload -p ui
```

**（可选）** dev 与 prod
`dx` 默认读取 `crates/ui/Dioxus.toml` 配置。仓库中还提供了 `Dioxus.toml.prod`，用于更好的生产构建输出。
如果你想了解两者差异，请参考官方 [DioxusLabs documentation](https://dioxuslabs.com)。

## CI - 代码质量

本仓库通过 Pull Request CI 检查来保证代码质量。

在你的贡献被合并前，以下 CI 任务必须全部通过：

- `Check`：`RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features`
- `Rustfmt`：`cargo fmt --all -- --check`
- `Clippy`：`cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports`

如果任一任务失败，请更新 PR，直到所有检查通过。

### 本地校验（推荐）

推送前，建议在本地执行同样的检查：

```sh
RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports
```

**（可选）** 本地 Git Hook（私有）
你可以在 `.git/hooks/pre-commit` 中创建本地私有 Hook，自动执行上述命令。

示例模板如下：

```bash
#!/usr/bin/env bash
set -euo pipefail

RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports
```

注意：`.git/hooks/` 下的文件仅在本地生效，不会提交到本仓库。
