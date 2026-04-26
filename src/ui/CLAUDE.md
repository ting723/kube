
[根目录](../../CLAUDE.md) > [src](../) > **ui**

# UI 模块

## 模块职责

TUI (终端用户界面) 渲染模块，负责所有用户界面的渲染、布局管理和组件展示。它使用 ratatui 库构建界面，crossterm 处理终端交互。

## 入口与启动

主要入口点是 `render_ui()` 函数，位于 `src/ui/mod.rs`。

```rust
// 在 main.rs 中调用
terminal.draw(|f| ui::render_ui(f, &app))?;
```

## 对外接口

### 主要函数

- `render_ui(f: &mut Frame, app: &AppState)` - 主 UI 渲染函数
  - 渲染头部（标签页）
  - 渲染主内容区域
  - 渲染页脚（快捷键提示）
  - 渲染命令行

### 子模块

- `layout.rs` - 布局管理
- `components/mod.rs` - 组件导出
- `components/*` - 各组件实现

## 组件列表

| 组件名称 | 文件 | 描述 |
|---------|------|------|
| NamespaceList | components/namespace_list.rs | 命名空间列表 |
| PodList | components/pod_list.rs | Pod 列表 |
| ServiceList | components/service_list.rs | Service 列表 |
| DeploymentList | components/deployment_list.rs | Deployment 列表 |
| JobList | components/job_list.rs | Job 列表 |
| DaemonSetList | components/daemonset_list.rs | DaemonSet 列表 |
| PVCList | components/pvc_list.rs | PVC 列表 |
| PVList | components/pv_list.rs | PV 列表 |
| NodeList | components/node_list.rs | Node 列表 |
| ConfigMapList | components/configmap_list.rs | ConfigMap 列表 |
| SecretList | components/secret_list.rs | Secret 列表 |
| Logs | components/logs.rs | Pod 日志查看器 |
| Describe | components/describe.rs | 资源描述查看器 |
| YamlView | components/yaml_view.rs | YAML 配置查看器 |
| TopView | components/top_view.rs | 资源监控视图 |
| Search | components/search.rs | 搜索界面 |
| Confirm | components/confirm.rs | 确认对话框 |
| Help | components/help.rs | 帮助信息 |

## 关键依赖与配置

- **ratatui** - TUI 渲染库
- **crossterm** - 终端交互
- **依赖关系**: 从 AppState 读取数据，只读依赖

## 设计模式

### 布局结构

UI 分为四个垂直区域:
1. 头部 (标签页)
2. 主内容区
3. 页脚 (快捷键提示)
4. 命令行

### 国际化

所有组件都支持中文/英文切换，通过 `app.language_chinese` 标志控制。

## 测试与质量

- 单元测试位置: 在组件模块中
- 测试策略: 主要通过手动测试 UI 渲染
- 使用方法: `cargo run` 进行交互式测试

## 常见问题 (FAQ)

### 如何添加新的组件？

1. 在 `src/ui/components/` 中创建新文件
2. 在 `src/ui/components/mod.rs` 中导出
3. 在 `AppMode` 中添加新的模式变体
4. 在 `render_ui()` 中添加匹配分支

### 如何自定义主题？

修改相应组件文件中的颜色和样式设置。

## 相关文件清单

| 文件路径 | 描述 |
|---------|------|
| src/ui/mod.rs | 主 UI 调度器 |
| src/ui/layout.rs | 布局管理 |
| src/ui/components/mod.rs | 组件导出 |
| src/ui/components/*.rs | 各组件实现 |

## 变更记录 (Changelog)

- **2025-11-01**: 初始化文档
