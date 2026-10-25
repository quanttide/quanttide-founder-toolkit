//! `skills` 的端到端测试：注册、装盘、锁文件、状态比对。
//!
//! 每条 Skill 都用最小正文：这个库只管搬运，不挑内容。

use quanttide_founder::agent::skills::{
    EmbeddedSkill, InstallOptions, Mode, Status, describe, install, install_all, skill_path, status,
};

const SKILL_MD: &str = "---\nname: demo\ndescription: 演示用 Skill\n---\n\n# Demo\n";

fn entry() -> EmbeddedSkill {
    EmbeddedSkill {
        id: "demo",
        skill_md: SKILL_MD,
        auxiliary: &[("references/note.md", "附属文件正文\n")],
    }
}

#[test]
fn 装到目标仓并写锁文件() {
    let temp = tempfile::tempdir().expect("临时目录");
    let options = InstallOptions::new(temp.path());
    let installed = install(&entry(), &options).expect("应能装上");

    assert_eq!(installed.id, "demo");
    assert_eq!(installed.path, temp.path().join(".agents/skills/demo"));
    assert!(installed.path.join("SKILL.md").is_file());
    assert!(installed.path.join("references/note.md").is_file());
    assert!(temp.path().join(".agents/.skill-lock.json").is_file());
}

#[test]
fn 状态分未装一致与不一致三种() {
    let temp = tempfile::tempdir().expect("临时目录");
    let root = temp.path();
    let entry = entry();

    assert_eq!(status(&entry, root), Status::Missing);

    install(&entry, &InstallOptions::new(root)).expect("应能装上");
    assert_eq!(status(&entry, root), Status::Current);

    std::fs::write(skill_path(root, entry.id), "改过的正文").expect("改得动");
    assert_eq!(status(&entry, root), Status::Stale);
}

#[test]
fn 名字对不上就拒绝() {
    let entry = EmbeddedSkill {
        id: "demo",
        skill_md: "---\nname: other\ndescription: 名字不一致\n---\n",
        auxiliary: &[],
    };
    let error = describe(&entry).expect_err("应当拒绝");
    assert!(error.contains("对不上"), "{error}");
}

#[test]
fn 缺出处的正文也拒绝() {
    let entry = EmbeddedSkill {
        id: "demo",
        skill_md: "# 没有 frontmatter\n",
        auxiliary: &[],
    };
    assert!(describe(&entry).is_err());
}

#[test]
fn 复制模式不依赖链接() {
    let temp = tempfile::tempdir().expect("临时目录");
    let mut options = InstallOptions::new(temp.path());
    options.mode = Mode::Copy;
    let installed = install(&entry(), &options).expect("应能装上");

    assert!(!installed.symlink_failed);
    assert!(installed.path.join("SKILL.md").is_file());
}

#[test]
fn 也投递到额外目录() {
    let temp = tempfile::tempdir().expect("临时目录");
    let mut options = InstallOptions::new(temp.path());
    options.target_dirs = vec![temp.path().join(".claude/skills")];
    install(&entry(), &options).expect("应能装上");

    assert!(temp.path().join(".claude/skills/demo/SKILL.md").is_file());
}

#[test]
fn 强制重装会清掉旧目录() {
    let temp = tempfile::tempdir().expect("临时目录");
    let root = temp.path();
    install(&entry(), &InstallOptions::new(root)).expect("应能装上");

    let stale = root.join(".agents/skills/demo/leftover.md");
    std::fs::write(&stale, "上一版留下的").expect("写得动");
    assert!(stale.is_file());

    let mut options = InstallOptions::new(root);
    options.force = true;
    install(&entry(), &options).expect("应能重装");
    assert!(!stale.exists(), "强制重装应清掉旧文件");
}

#[test]
fn 一批里一条失败不影响其他条() {
    let temp = tempfile::tempdir().expect("临时目录");
    let good = entry();
    let bad = EmbeddedSkill {
        id: "bad",
        skill_md: "---\nname: bad\n---\n",
        auxiliary: &[],
    };
    let results = install_all(&[good, bad], &InstallOptions::new(temp.path()));
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    assert!(temp.path().join(".agents/skills/demo/SKILL.md").is_file());
}

#[test]
fn 读得到名字与说明() {
    let summary = describe(&entry()).expect("应能读");
    assert_eq!(summary.id, "demo");
    assert_eq!(summary.name, "demo");
    assert_eq!(summary.description, "演示用 Skill");
}
