//! 量潮创始人工具箱（Rust 包）。
//!
//! 业务代码已归档至 `data/archive/toolkit/rust/`；重建时在 `src/` 下按
//! core / memory / fiction 分层添加，并在此导出。
//!
//! 当前只有 `agent` 一层：把编译期嵌入的 Agent Skill 装进目标仓。

pub mod agent;

pub use agent::skills::{EmbeddedSkill, InstallOptions, Installed, Mode, Status, Summary};

/// 包版本，取自 `Cargo.toml`。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
