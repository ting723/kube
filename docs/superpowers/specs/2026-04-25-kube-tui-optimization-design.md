# Kube TUI 优化重构设计文档

**项目:** Kube TUI - Kubernetes 终端 UI 工具  
**日期:** 2026-04-25  
**作者:** AI-assisted design  

## 概述

Kube TUI 是一个用 Rust 编写的 Kubernetes 终端 UI 管理工具，灵感来自 lazydocker。本设计文档描述了全面优化重构方案，目标：

- 使用最新 Rust 版本（1.95.0）
- 代码架构更加清晰简洁
- 功能更加稳定
- 单元测试全覆盖（目标 80%+ 覆盖率）
- 新增多个用户体验增强功能

## 设计决策汇总

| 项目 | 决策 |
|------|------|
| Rust 版本 | 升级到 **1.95.0**（最新稳定版）|
| 优化范围 | 架构重构 + 依赖更新 + 测试全覆盖 + 新增六个功能 |
| 架构策略 | **完整模块化重构** - 按职责彻底分组 |
| 测试组织 | **分离式测试** - 测试代码放在 `tests/` 目录，`src/` 保持干净 |
| 新增功能 | 全部六个：快速上下文切换、批量操作、Port-forward、日志搜索、tail -f 流式日志、命令历史 |

## 架构重构

### 重构前

```
src/
├── main.rs          (838 行) - 入口 + 主事件循环 + 大量数据加载逻辑
├── app.rs           (1233 行) - AppState + AppMode + 所有键盘事件处理 + 搜索
├── events.rs        事件轮询
├── errors.rs        错误类型
├── resource_type.rs 资源枚举
├── kubectl/         (well separated ✓)
└── ui/             (well separated ✓)
```

问题：
- `main.rs` 和 `app.rs` 过大，职责不清晰
- 不符合单一职责原则
- 难以测试

### 重构后

```
src/
├── main.rs         (~80 行) - only: 初始化 + 启动，不包含业务逻辑
├── app/
│   ├── mod.rs      模块导出
│   ├── state.rs    AppState + AppMode + ConfirmAction - 状态定义
│   ├── config.rs   用户配置读写（收藏、历史）
│   ├── key_handler.rs  键盘事件处理（顶级分发）
│   └── handlers/   各模式具体按键处理
│       ├── mod.rs
│       ├── list_mode.rs
│       ├── logs_mode.rs
│       ├── detail_mode.rs
│       └── batch.rs 批量操作处理
├── events.rs       保持不变 - 事件轮询
├── errors.rs       保持不变 - 错误类型定义
├── resource_type.rs 保持不变 - 资源类型枚举
├── kubectl/        保持现有结构，内部优化
│   ├── mod.rs
│   ├── client.rs
│   ├── commands.rs
│   └── types.rs
└── ui/
    ├── mod.rs      render_ui - 保持不变
    ├── layout.rs   布局 - 保持不变
    └── components/ 保持现有每个组件一个文件的结构
```

### 模块职责划分

| 模块 | 职责 |
|------|------|
| `main.rs` | 终端初始化、启动、退出恢复，只做这三件事 |
| `app::state` | 所有应用状态定义，默认值，工具方法（如 `should_refresh`） |
| `app::config` | 用户配置（收藏命名空间、命令历史）的读写、持久化 |
| `app::key_handler` | 顶级键盘事件分发，调用对应模式处理器 |
| `app::handlers::*` | 各具体模式的键盘事件处理逻辑 |
| `events` | 事件轮询（键盘、鼠标、resize）|
| `kubectl/*` | Kubernetes 客户端、命令构建、类型定义 |
| `ui/*` | UI 渲染、组件绘制 |

## 测试策略

### 目录结构

```
tests/
├── app/
│   ├── state_test.rs
│   ├── config_test.rs
│   └── key_handler_test.rs
├── kubectl/
│   ├── client_test.rs
│   ├── commands_test.rs
│   └── types_test.rs
├── events_test.rs
├── resource_type_test.rs
├── ui/
│   └── ... (components 对应测试)
└── test_utils/
    └── mod.rs  测试通用工具函数
```

### 测试覆盖目标

- 所有纯逻辑函数必须有单元测试
- IO 操作使用 mock 进行测试
- 目标覆盖率：**80%+**
- 集成测试放在 `tests/integration/`

### 依赖注入便于测试

- `KubectlClient` 定义 trait，方便 mock 进行单元测试
- 配置读写抽象，便于测试

## 依赖版本更新

- Rust: `1.85` → `1.95.0`
- edition: 保持 `2024`（已经是最新）
- 更新所有依赖到最新兼容版本：
  - `ratatui` 0.29 → latest
  - `crossterm` 0.29 → latest
  - `tokio` 1 → latest（保持 1.x）
  - `serde` 1 → latest
  - `anyhow` 1 → latest
  - `thiserror` 2 → latest（已经是 2）
  - `clap` 4 → latest
  - `chrono` 0.4 → latest
  - `color-eyre` 0.6 → latest

## 新增功能详细设计

### 1. 快速上下文切换

**功能描述：** 允许用户收藏常用命名空间，快速切换，记住列表位置。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub favorite_namespaces: Vec<String>,
  pub last_selected_positions: HashMap<AppMode, usize>,
  ```
- 新建 `src/app/config.rs`：
  - 读取 `~/.config/kube-tui/config.json`
  - 写回配置（退出时）
  - 配置结构：
    ```json
    {
      "favorite_namespaces": ["default", "kube-system"],
      "command_history": ["kubectl delete pod ...", ...],
    }
    ```
- `src/ui/components/namespace_list.rs`：
  - 排序：收藏的命名空间排在最前面
  - 显示星形标记区分收藏

- **快捷键：** `F` 在 NamespaceList 模式下 - 收藏/取消收藏当前

### 2. 批量操作

**功能描述：** 支持多选资源，批量删除。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub batch_mode: bool,
  pub selected_batch_items: HashSet<usize>,
  ```

- **快捷键：**
  - `Ctrl+Space` - 切换批量模式，选中当前项
  - `D` - 触发批量删除确认

- `src/ui/components/*_list.rs`：
  - 批量模式下，选中项目用不同背景色高亮

- `src/app/handlers/batch.rs`：
  - 执行批量删除操作

### 3. Port-forward 支持

**功能描述：** 在 TUI 内直接启动 `kubectl port-forward`，方便本地访问 Service/Pod。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub active_portforwards: Vec<std::process::Child>,
  ```

- **快捷键：** `P` 在 PodList/ServiceList - 弹出端口输入框

- 新建 `src/ui/components/portforward_dialog.rs`：
  - 用户输入本地端口:远程端口
  - 确认后启动进程

- `src/kubectl/commands.rs` 新增 `portforward()` 命令构建

- 退出程序时，遍历 `active_portforwards` ，调用 `kill()` 终止所有进程

- 状态栏显示活动 port-forward 信息

### 4. 日志搜索

**功能描述：** 在日志视图中搜索关键词，快速定位。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub log_search_query: String,
  pub log_search_results: Vec<usize>,  // 匹配的行号
  pub current_log_search_index: usize,
  pub log_search_mode: bool,
  ```

- **快捷键：** `/` 在 Logs 模式 - 进入搜索模式
  - `n` - 下一个匹配
  - `N` - 上一个匹配

- `src/ui/components/logs.rs`：
  - 匹配行高亮显示
  - 底部显示 `X/Y matches`

- 复用现有搜索框组件逻辑，不需要重新实现

### 5. 滚动日志模式 (tail -f)

**功能描述：** 真正流式日志输出，类似 `kubectl logs -f`，而不是定时拉取。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub streaming_logs: bool,  // true = 流式, false = 定时拉取
  ```

- `src/kubectl/client.rs` 新增方法：
  ```rust
  pub async fn stream_pod_logs(...) -> Result<impl Stream<Item = String>>
  ```
  - 使用 `tokio::process::Command` 启动 `kubectl logs -f`
  - 异步读取 stdout，每一行输出发送到 AppState

- `src/main.rs` 主循环：
  - 后台任务持续读取，追加到 `app.logs`
  - 自动滚动到底部如果 `logs_auto_scroll` 开启

- **快捷键：** `F` 在 Logs 模式 - 切换流式/拉取模式

### 6. 命令历史

**功能描述：** 记录用户执行过的命令（delete、exec、port-forward），可以快速重复执行。

**技术方案：**

- `src/app/state.rs` `AppState` 新增：
  ```rust
  pub command_history: Vec<String>,
  ```

- 最大保存 100 条历史，保存在配置文件中
- **快捷键：** `H` - 打开命令历史弹窗

- 新建 `src/ui/components/command_history.rs`：
  - 列表显示历史命令
  - 选中后重新执行该命令

- 只记录用户主动发起的操作命令，不记录自动刷新

## 新增文件清单

| 路径 | 描述 |
|------|------|
| `src/app/mod.rs` | 新建 - 模块导出 |
| `src/app/state.rs` | 新建 - AppState 和 AppMode 定义 |
| `src/app/config.rs` | 新建 - 配置读写 |
| `src/app/key_handler.rs` | 新建 - 键盘事件处理 |
| `src/app/handlers/mod.rs` | 新建 |
| `src/app/handlers/list_mode.rs` | 新建 - 列表模式处理 |
| `src/app/handlers/logs_mode.rs` | 新建 - 日志模式处理 |
| `src/app/handlers/detail_mode.rs` | 新建 - 详情模式处理 |
| `src/app/handlers/batch.rs` | 新建 - 批量操作 |
| `src/ui/components/portforward_dialog.rs` | 新建 - port-forward 对话框 |
| `src/ui/components/command_history.rs` | 新建 - 命令历史弹窗 |
| `tests/` | 新建目录 - 所有单元测试 |
| `tests/test_utils/mod.rs` | 新建 - 测试工具 |

## 重构修改清单

| 原有文件 | 修改内容 |
|----------|----------|
| `Cargo.toml` | 更新 rust-version，更新所有依赖版本 |
| `src/main.rs` | 重构 - 移除大段逻辑，只保留入口 |
| `src/app.rs` | 完全移除，内容拆分到 `app/` 下多个文件 |
| `src/kubectl/client.rs` | 添加 streaming log 方法 |
| `src/kubectl/commands.rs` | 添加 port-forward 命令构建 |
| `src/ui/components/namespace_list.rs` | 修改 - 收藏排序显示 |
| `src/ui/components/logs.rs` | 修改 - 添加日志搜索高亮 |
| `src/events.rs` | 少量调整，保持不变 |
| `src/errors.rs` | 添加新错误类型 |
| `src/resource_type.rs` | 保持不变 |

## 验收标准

1. **编译通过**：`cargo build` 无错误
2. **测试通过**：`cargo test` 全部通过，覆盖率 ≥ 80%
3. **Clippy 检查通过**：`cargo clippy` 无警告
4. **功能完整**：所有六个新增功能正常工作
5. **架构清晰**：每个模块职责单一，符合 SOLID 原则
6. **代码简洁**：使用 Rust 1.95 最新特性，代码更简洁

## 下一步

设计评审通过后，调用 `writing-plans` 技能生成详细实施计划。
