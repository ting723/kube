# Kube TUI - Kubernetes Terminal Interface

一个类似 lazydocker 的 Kubernetes 终端工具，使用 Rust 编写。

## 功能特性

- 🔍 **命名空间浏览**: 快速切换和查看不同命名空间
- 📦 **Pod 管理**: 查看、操作和管理 Pod
- 🔧 **服务管理**: 查看和管理 Kubernetes 服务
- 📋 **日志查看**: 实时查看 Pod 日志 (支持自动刷新和滚动)
- 📄 **资源描述**: 查看资源的详细信息 (支持 YAML 语法高亮)
- 📊 **YAML 配置查看**: 完整查看资源的 YAML 配置
- 💻 **资源监控**: 查看 Pod 的 CPU 和内存使用情况
- 🔍 **智能搜索**: 支持实时搜索和模糊匹配
- ⚡ **自动刷新**: 自动更新状态信息
- 🎨 **彩色界面**: 直观的状态颜色显示和语法高亮
- 🖱️ **鼠标支持**: 支持鼠标文字选择和滚轮操作

## 依赖要求

- Rust 1.70+ 
- kubectl 命令行工具 (已配置并能访问集群)
- 配置好的 kubeconfig 文件

## 安装

### 从源码构建

```bash
git clone <repository-url>
cd kube
cargo build --release
```

构建完成后，二进制文件位于 `target/release/kube-tui`

### 运行

确保 kubectl 已正确配置：

```bash
kubectl cluster-info
```

然后运行 Kube TUI：

```bash
./target/release/kube-tui
```

## 使用说明

### 界面布局

```
┌─ Namespaces ─┬─ Resources ────────────────────────────────────┐
│ default      │ PODS                                           │
│ kube-system  │ ┌──────────────────────────────────────────────┤
│ kube-public  │ │ NAME          READY   STATUS    RESTARTS AGE │
│              │ │ pod-1         1/1     Running   0        1d  │
│              │ │ pod-2         0/1     Pending   0        5m  │
├──────────────┼─────────────────────────────────────────────────┤
│ Actions      │ [l]ogs [d]escribe [e]xec [Enter]select        │
└──────────────┴─────────────────────────────────────────────────┘
```

### 快捷键

#### 通用操作
- `↑/↓` 或 `j/k` - 导航列表
- `Tab` - 在面板间切换
- `Enter` - 选择项目/进入模式
- `Esc` - 返回上级/退出模式
- `q` - 退出应用
- `?` - 显示帮助

#### 命名空间视图
- `Enter` - 切换到选中的命名空间

#### Pod 视图
- `Space` - 查看 Pod 详细描述
- `Y` - 查看 Pod 完整 YAML 配置
- `T` - 查看 Pod 资源使用情况 (CPU/内存)
- `L` - 查看 Pod 日志
- `E` - 进入 Pod (打开新终端)
- `D` - 删除 Pod (需要确认)
- `/` - 搜索 Pod

#### 服务视图
- `Space` - 查看服务详细描述
- `Y` - 查看服务 YAML 配置
- `D` - 删除服务
- `/` - 搜索服务

#### 其他资源视图 (Node/ConfigMap/Secret/PVC/PV/Job/DaemonSet/Deployment)
- `Space` - 查看资源详细描述
- `Y` - 查看资源 YAML 配置
- `D` - 删除资源 (适用的资源类型)
- `/` - 搜索资源

#### 日志视图
- `J/K` - 滚动日志
- `PgUp/PgDn` - 翻页
- `A` - 切换自动滚动
- `R` - 切换自动刷新
- `Esc` - 返回 Pod 列表

#### YAML/描述/Top 视图
- `J/K` - 滚动内容
- `PgUp/PgDn` - 翻页
- `Esc` - 返回上一级

### 状态颜色说明

- 🟢 **绿色**: Running (运行中)
- 🟡 **黄色**: Pending (等待中)
- 🔴 **红色**: Failed/Error (失败/错误)
- 🔵 **蓝色**: Succeeded (成功)
- ⚪ **灰色**: Unknown (未知)

## 功能详情

### 自动刷新
应用每 5 秒自动刷新数据，确保显示的信息是最新的。

### 错误处理
- 检查 kubectl 可用性
- 友好的错误信息显示
- 网络超时处理

### 跨平台支持
支持以下平台：
- Linux
- macOS  
- Windows

## 开发

### 项目结构

```
src/
├── main.rs              # 程序入口
├── app.rs               # 应用状态管理
├── events.rs            # 事件处理
├── kubectl/             # kubectl 命令封装
│   ├── mod.rs
│   ├── client.rs        # kubectl 客户端
│   ├── types.rs         # K8s 资源类型
│   └── commands.rs      # 命令定义
└── ui/                  # UI 相关
    ├── mod.rs
    ├── layout.rs        # 布局管理
    └── components/      # UI 组件
        ├── namespace_list.rs
        ├── pod_list.rs
        ├── service_list.rs
        ├── logs.rs
        ├── describe.rs
        └── help.rs
```

### 依赖库

- `ratatui` - 终端用户界面
- `crossterm` - 跨平台终端操作
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `anyhow` - 错误处理
- `chrono` - 时间处理

## 故障排除

### kubectl 未找到
```
Error: kubectl is not available in PATH
```
**解决方案**: 安装 kubectl 并确保在 PATH 中可访问

### 集群连接失败
**解决方案**: 
1. 检查 kubeconfig 配置
2. 验证集群连接: `kubectl cluster-info`
3. 检查网络连接

### 权限错误
**解决方案**: 确保当前用户有足够的 RBAC 权限访问 Kubernetes 资源

## 贡献

欢迎提交 Issues 和 Pull Requests！

## 许可证

MIT License

## 致谢

灵感来源于 [lazydocker](https://github.com/jesseduffield/lazydocker)