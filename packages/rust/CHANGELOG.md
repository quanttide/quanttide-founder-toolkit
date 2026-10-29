# Changelog

## [Unreleased]

### Changed

- `ROADMAP.md`：移至包根与 README 并列，接续新路线图——core/exit 退出码四码（0/1/2/3，2 用法错、3 对象缺席），core 为核心层；`docs/dev-guide/` 随之移除；上一份语义建模路线图随代码归档，README 指向同步

## [0.1.0-alpha.5] - 2026-10-25

### Changed

- 模块名统一用单数：`agent::skills` 改为 `agent::skill`，文件 `src/agent/skills.rs` 改为 `src/agent/skill.rs`，集成测试同改；包根导出路径同步

## [0.1.0-alpha.4] - 2026-10-25

### Added

- `install_report`：同 `install`，多返回一句锁文件提示

### Fixed

- 系统位置已有一份别的工具写的 `.skill-lock.json` 时不再拦安装：锁文件认不出来就跳过记录，Skill 照装，认不出的文件原样不动

## [0.1.0-alpha.3] - 2026-10-25

`agent` 一层落地：把编译期嵌入的 Skill 装进目标仓。

### Added

- **`agent::skills` 模块**：把编译期嵌入的 Agent Skill 装进目标仓。`EmbeddedSkill` 登记正文与附属文件，`install` 交给 `agent-skills-rs` 写 `<root>/.agents/skills/<name>`、可选投递到 `.claude/skills`，并维护 `.agents/.skill-lock.json`；`status` 把磁盘正文与嵌入内容逐字比，给出未装／一致／不一致
- 校验：`id` 与 SKILL.md 的 `name` 不符在注册时拒绝；缺 frontmatter 同样拒绝
- 测试：装盘与锁文件、三种状态、复制模式、额外投递目录、强制重装清旧文件、一条失败不影响同批

### Changed

- 模块从 `src/skills.rs` 移到 `src/agent/skills.rs`，入口为 `quanttide_founder::agent::skills`；包根仍导出主要类型
- 新增依赖 `agent-skills-rs` 0.3.1

## [0.1.0-alpha.2] - 2026-10-02

把执行接上设计：九处语义降级（字符串匹配、恒等返回、占位分支）换成对 core 接口的调用，判据从代码搬进 YAML。详见 [ROADMAP.md](docs/dev-guide/roadmap.md) 与 [docs/criteria.md](docs/criteria.md)。

### Added

- **判据**：`docs/criteria.md`（九处判断的意图/判据/边界）；`Artifact` 增 `name`（类别 key）、`criteria`（意图/判据/正例/反例/易混例）、`grades`（分级类别）；`Workflow` 增 `actions`（合并策略名单）
- **`memory/rules.rs` / `fiction/rules.rs`**：内置规则资产，编译期嵌入 toolkit 根 `tests/fixtures/` 的共享 YAML
- **`JournalEntry::tags` / `Tag`**：tag 从元数据提取（日期从文件名、来源从路径），与语义判断分开
- **`Engine::extract`**：域内语义填表入口；`ExtractionResult.fields` 承接规则 `extract` 列出的字段
- **测试**：`semantic_llm` 按判据回 JSON（覆盖 LLM 首选路径），`demo_llm` 回纯文本（覆盖规则降级路径）；换 YAML 换解读、共享资产端到端读取两组新钉子

### Changed

- **分类**：`Destination` 枚举拆掉，去向是 `Artifact.name`（journal 是补集）；`route` + `classify` 合并为单条 `classify(engine, text, categories)`，判据交给 `Engine::judge`，理由必填
- **判断**：`grade` / `cluster` / `is_same_essence` / `is_refuted` / `decide_merge` / `should_promote_to_profile` / `should_fork_to_roadmap` 吃 `&Engine`——LLM 按 YAML 判据首选，失败走显式规则降级
- **fiction 三步**：取样、观察展开、落实片段、`extract_packaging` 同样吃 `&Engine`，状态机的上下文即 `Engine`（`handle_with_context`）
- **`Engine::judge`**：结构化进出，输出必须是 `{decision, reason}`，缺 `reason` 视为错误
- **`Judgment.reason` / `ClassifiedEntry.reason`**：`Option<String>` 改必填
- **`InsightGrade` / `MergeAction`**：枚举改数据驱动（名字来自 `grades` / `actions`）
- **固定取值**：`split_by` / `title.source` / `description.location` 改枚举，加载期校验；`Step.verb` 改 `Verb`，未知动词加载即报错
- **`Engine::scan` 步**：候选进 `WorkflowResult.candidates`，不再清空 `items`；关键词从 workflow 的 `rules` 文本读出
- **`RuleBasedExtractor`**：说明取哪个位置由 `description.location` 配置（`none` 不提取）
- **示例**：`memory_workflow` 演示只判前 10 行（全量逐行是真实 LLM 调用）；未配置 `LLM_API_KEY` 时用演示客户端走降级，离线可跑

### Removed

- **关键词表**：`COGNITION` / `INTENT` / `DIRECTION` / `PROFILE` / `ALL_KEYWORDS`、13 个单字动词、分词重合度、「不是」子串判断——语义判断不再退化为字符串包含
- **`route` / `Destination` / `Error::UnknownVerb`**：由数据驱动分类与加载期校验取代

### 资产迁移

- 共享 YAML 从 `packages/dart/assets/` 迁至 toolkit 根 `tests/fixtures/`（两包共用，同一件事只写一处）

## [0.1.0-alpha.1] - 2026-09-27

首个 alpha 预发布，行为对齐同日发布的 Dart 包 `quanttide_founder` 0.1.0-alpha.1。

### Added

- **core/**：parse（Markdown 解析 + 结构切分）、rules（Artifact / Workflow YAML 加载）、engine（scan / judge / merge + 规则/LLM 两个提取器）
- **memory/**：models（JournalEntry / InsightDoc / ProfileDoc / RoadmapDoc）、repository（MemoryRepository）、states（四步流程状态机 + 三问路由、证据分级、聚类、合并策略、升降级）
- **fiction/**：models（Novel / Chapter / Stage / Observation）、repository（FictionRepository）、states（三步提炼状态机 + 章节编号轴、阶段流转、包装文案）
- **example/**：解析 + 工作流四入口（parse_memory / parse_fiction / memory_workflow / fiction_workflow）+ 订阅演示（agent_subscribe：Agent 驱动任务状态机，进度/流水/闸门提示各自订阅状态流，`after_transition` 钩子推快照）
- **tests/**：parse / rules / engine / memory / fiction 五组行为钉子

### 与 Dart 包的差异

- **LLM**：不复刻 `LlmClient` 接口，改用 quanttide-agent 的 `LLM`——`Engine::judge`、`LlmExtractor` 直接用它发请求，响应 JSON 剥壳走 `quanttide_agent::parse_structured_output`
- **编排**：Dart 的 bloc 换成 statig 状态机，模块名 `memory/states.rs`、`fiction/states.rs`；四步/三步流程是事件驱动的状态转移（`Scan/Route/Cluster`、`Sample/Expand/Settle`），判断规则收成自由函数
- **流程参数**：`MemoryFlow::process_journal` 不再收 Dart 侧未参与计算的 `Workflow` 参数，流程定义即状态机本身
- **日期正则**：日志文件名匹配用行尾锚定 `\.md$`——Dart 侧 `RegExp(r'...\.md\$')` 把 `$` 写成了字面量，永远匹配不到 `YYYY-MM-DD.md`，`MemoryRepository.load` 在 Dart 侧读不出日志（待修）
