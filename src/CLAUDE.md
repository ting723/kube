
[根目录](../CLAUDE.md) > **src**

# src 根模块

## 模块职责

这是 Kube TUI 应用的核心模块，包含所有应用逻辑、状态管理和协调各个子模块。

## 模块结构

```
src/
├── main.rs              # 应用入口，主事件循环
├── app.rs               # 应用状态管理
├── events.rs            # 事件轮询处理
├── errors.rs            # 错误类型定义
├── resource_type.rs     # 资源类型枚举
├── kubectl/             # Kubernetes 客户端模块
│   ├── mod.rs
│   ├── client.rs
│   ├── commands.rs
│   └── types.rs
└── ui/                  # UI 渲染模块
    ├── mod.rs
    ├── layout.rs
    └── components/
```

## 核心模块

### main.rs

应用入口点，职责包括：
- 初始化错误处理
- 检查 kubectl 可用性
- 设置终端模式
- 创建应用状态
- 加载初始数据
- 运行主事件循环
- 恢复终端状态

### app.rs

定义核心应用状态和模式，包括：

- `AppMode` 枚举 - 所有应用模式（各种资源列表、日志、描述等）
- `ConfirmAction` 枚举 - 确认对话框操作
- `AppState` 结构体 - 主要应用状态
  - 状态管理
  - 事件处理
  - 滚动逻辑
  - 搜索功能
  - 刷新控制

### events.rs

事件轮询模块，负责：
- 键盘事件
- 鼠标事件
- 终端调整大小事件

### errors.rs

应用特定错误类型，使用 `thiserror` 定义。

### resource_type.rs

资源类型枚举 `ResourceType`，提供：
- kubectl 资源名称
- 单复数形式
- 命名空间需求判断
- 与 AppMode 的转换

## 入口与启动

```rust
# 运行应用
cargo run

# 或使用 Makefile
make dev
```

启动流程：
1. 检查 kubectl 可用性
2. 初始化终端（备用屏幕、原始模式）
3. 创建 AppState
4. 加载初始数据（命名空间、Pod、Service）
5. 进入主事件循环

## 关键依赖与配置

- **tokio** - 异步运行时
- **ratatui** - TUI 渲染
- **crossterm** - 终端交互
- **anyhow** - 错误处理
- **thiserror** - 错误定义
- **clap** - 命令行参数
- **serde** - 序列化
- **chrono** - 时间处理

## 应用流程

1. 初始化 → 加载命名空间
2. 用户导航 → 选择命名空间
3. 浏览资源 → 列表视图
4. 资源操作 → 查看描述、日志、YAML、执行命令
5. 循环 → 自动刷新数据

## 测试与质量

- 单元测试: `cargo test`
- 功能测试: `make test-all`
- 代码检查: `make lint`
- 格式化: `make format`

## 相关文件清单

| 文件路径 | 描述 |
|---------|------|
| src/main.rs | 应用入口 |
| src/app.rs | 状态管理 |
| src/events.rs | 事件处理 |
| src/errors.rs | 错误定义 |
| src/resource_type.rs | 资源类型枚举 |
| src/kubectl/ | Kubernetes 客户端模块 |
| src/ui/ | UI 渲染模块 |

## 变更记录 (Changelog)

- **2025-11-01**: 初始化文档
