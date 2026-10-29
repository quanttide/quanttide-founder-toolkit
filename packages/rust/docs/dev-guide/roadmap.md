# ROADMAP

Rust 包（quanttide_founder）路线图。上一份路线图（语义建模重构）2026-10-02 验收通过，其代码与包内文档已随 2026-10-17 归档进 `data/archive/toolkit/`，本文件接续。`[x]` = 已完成。

## core/exit：退出码四码

CLI 的退出码是 API 五面（命令、输出、退出码、落盘、契约）之一，qtfounder CLI 打 `cli/v0.1.0` 前要冻结，冻结后改码即破坏性变更。语义在本包单点定义，CLI 只消费——将来再有第二个 CLI 也用同一套，不会再长出第二套码。

### 范围

- [ ] `src/core/exit.rs`：四个常量，语义写进文档注释——`OK = 0` 跑通无命中、`FINDINGS = 1` 跑通有命中、`USAGE = 2` 用法或环境错、`NOT_FOUND = 3` 目标缺席；2 与 3 的分界一句话：输入写法没错、对象缺席 = 3
- [ ] `core` 为核心层：`lib.rs` 增 `pub mod core;`，后续核心常量与通用类型归这层
- [ ] 测试钉住四个值互不相同且恰为 0..=3——谁改常量的值，`cargo test` 直接红，这就是冻结的测试锚点
- [ ] 选常量不选枚举：`std::process::exit` 吃的就是 `i32`，枚举还得再转一层

### 边界

- Dart 侧不做：退出码是 CLI 独有概念，Dart 包没有 CLI 消费方；两侧同构承诺管的是业务模型，不摊这条

### 发布（需维护者定夺）

- [ ] 版本号维护者定（新增模块属兼容新增），CHANGELOG 记 Added
- [ ] 推 crates.io，不自动执行

### 消费：qtfounder CLI（在 CLI 仓执行，此处只记依赖与验收）

- [ ] CLI 升依赖到新版本后对齐全仓：`main.rs` 的 `exit(2)`、`docs` 的 0/1/2、`product` 的 2/3 改用常量
- [ ] 「目标缺席」对码：`docs format <不存在的文件>` 由 Err→2 改为 3；各命令测试补码值断言
- [ ] CLI `CONTRIBUTING.md` 补四码表（正本是 `core/exit.rs` 的文档注释）

### 验收

本包四绿（build / test / fmt / clippy）且包在 crates.io 可拉；CLI 侧 `cargo test` 全绿、四个出口被断言锁住。
