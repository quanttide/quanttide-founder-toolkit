//! Agent 这一侧：把 Skill 装到目标仓。
//!
//! 目前只有 `skill` 一层：编译期嵌入正文、写盘、按内容比对状态。
//! 这里管的是「给 Agent 用什么」，不判断 Skill 写得好不好。

pub mod skill;

pub use skill::{EmbeddedSkill, InstallOptions, Installed, Mode, Status, Summary, install_report};
