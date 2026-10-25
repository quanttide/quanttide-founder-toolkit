//! 把编译期嵌入的 Skill 装进目标仓的 `.agents/skills`。
//!
//! 宿主在自己的二进制里登记一批 [`EmbeddedSkill`]，[`install`] 交给
//! `agent-skills-rs` 写盘：canonical 目录是 `.agents/skills/<name>`，需要时再
//! 链接到 `.claude/skills` 这类 Agent 首选路径，并维护 `.agents/.skill-lock.json`。
//! Skill 正文随二进制走，目标仓不必联网，也不必知道源码树在哪。
//!
//! 这里只做搬运：Skill 写什么、装到哪由宿主决定；本库不判断内容好坏。

use std::path::{Path, PathBuf};

use agent_skills_rs::{
    InstallConfig, InstallMode, LockManager, Skill, Source, SourceType, install_skill,
    register_embedded_skill,
};

/// 一个编译期嵌入的 Skill。
///
/// `id` 必须与 SKILL.md frontmatter 里的 `name` 一致，不一致就在注册时报错——
/// 目录名与声明名各说各的，装完之后没人知道哪个才是真的。
pub struct EmbeddedSkill {
    pub id: &'static str,
    pub skill_md: &'static str,
    /// 附属文件：`(相对路径, 文本内容)`。二进制文件不进这里。
    pub auxiliary: &'static [(&'static str, &'static str)],
}

/// 安装方式。链接失败时会退回复制。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Link,
    Copy,
}

/// 一次安装的配置。
#[derive(Debug, Clone)]
pub struct InstallOptions {
    /// 目标仓根目录：装到 `<root>/.agents/skills`。
    pub root: PathBuf,
    pub mode: Mode,
    /// 同时投递的目录，如 `<root>/.claude/skills`。
    pub target_dirs: Vec<PathBuf>,
    /// 先清掉已装的同名目录，再写。删掉的附属文件不会留下残骸。
    pub force: bool,
}

impl InstallOptions {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            mode: Mode::Link,
            target_dirs: Vec::new(),
            force: false,
        }
    }
}

/// 装完之后的落点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    pub id: String,
    pub path: PathBuf,
    /// 链接不可用、退回复制时为真。
    pub symlink_failed: bool,
}

/// 已装状态：拿磁盘上的 SKILL.md 与嵌入内容逐字比。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Missing,
    Current,
    Stale,
}

impl Status {
    pub fn key(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Missing => "未安装",
            Self::Current => "已是最新",
            Self::Stale => "与嵌入版本不一致",
        }
    }
}

/// Skill 的自我介绍，列表用。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub id: String,
    pub name: String,
    pub description: String,
}

fn parse(entry: &EmbeddedSkill) -> Result<Skill, String> {
    let skill = register_embedded_skill(entry.skill_md, entry.auxiliary)
        .map_err(|error| format!("Skill「{}」注册失败：{error:#}", entry.id))?;
    if skill.name != entry.id {
        return Err(format!(
            "Skill「{}」的名字对不上：SKILL.md 里写的是「{}」",
            entry.id, skill.name
        ));
    }
    Ok(skill)
}

/// 读一条 Skill 的名字与说明。
pub fn describe(entry: &EmbeddedSkill) -> Result<Summary, String> {
    let skill = parse(entry)?;
    Ok(Summary {
        id: entry.id.to_string(),
        name: skill.name,
        description: skill.description,
    })
}

/// 装一条 Skill，并记进锁文件。
///
/// 锁文件写不进去不算失败：系统位置可能已有一份别的工具写的锁文件，格式我们认不了；
/// 那种情况下 Skill 照装，调用方用 [`install_report`] 取回一句提示。
pub fn install(entry: &EmbeddedSkill, options: &InstallOptions) -> Result<Installed, String> {
    install_report(entry, options).map(|(installed, _)| installed)
}

/// 同 [`install`]，多给一句锁文件提示；提示为空表示锁文件已更新。
pub fn install_report(
    entry: &EmbeddedSkill,
    options: &InstallOptions,
) -> Result<(Installed, Option<String>), String> {
    let skill = parse(entry)?;
    let canonical_dir = options.root.join(".agents").join("skills");
    let canonical = canonical_dir.join(&skill.name);
    if options.force && canonical.exists() {
        std::fs::remove_dir_all(&canonical)
            .map_err(|error| format!("清不掉旧目录 {}：{error}", canonical.display()))?;
    }

    let mut config = InstallConfig::new(canonical_dir);
    config.mode = match options.mode {
        Mode::Link => InstallMode::Symlink,
        Mode::Copy => InstallMode::Copy,
    };
    config.target_dirs = options.target_dirs.clone();
    config.fallback_to_copy = true;

    let result = install_skill(&skill, &config).map_err(|error| format!("{error:#}"))?;

    let lock_path = options.root.join(".agents").join(".skill-lock.json");
    let note = LockManager::new(lock_path)
        .update_entry(&skill.name, &embedded_source(), &result.path)
        .err()
        .map(|error| format!("锁文件没更新：{error:#}"));

    Ok((
        Installed {
            id: skill.name,
            path: result.path,
            symlink_failed: result.symlink_failed,
        },
        note,
    ))
}

/// 装一批：一条失败不影响其他条，按登记顺序返回结果。
pub fn install_all(
    entries: &[EmbeddedSkill],
    options: &InstallOptions,
) -> Vec<Result<Installed, String>> {
    entries
        .iter()
        .map(|entry| install(entry, options))
        .collect()
}

/// 目标仓里这一条装到哪。
pub fn skill_path(root: &Path, id: &str) -> PathBuf {
    root.join(".agents")
        .join("skills")
        .join(id)
        .join("SKILL.md")
}

/// 拿磁盘上的正文与嵌入内容比。没装、内容一致、内容不一致，三种。
pub fn status(entry: &EmbeddedSkill, root: &Path) -> Status {
    match std::fs::read_to_string(skill_path(root, entry.id)) {
        Err(_) => Status::Missing,
        Ok(text) if text == entry.skill_md => Status::Current,
        Ok(_) => Status::Stale,
    }
}

fn embedded_source() -> Source {
    Source {
        source_type: SourceType::Self_,
        url: None,
        subpath: None,
        skill_filter: None,
        ref_: None,
    }
}
