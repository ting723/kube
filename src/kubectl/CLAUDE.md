
[根目录](../../CLAUDE.md) > [src](../) > **kubectl**

# kubectl 模块

## 模块职责

Kubernetes API 交互模块，负责与 Kubernetes 集群通信，封装 kubectl 命令的执行和结果解析。

## 入口与启动

主要入口点是 `KubectlClient` 结构体，位于 `src/kubectl/client.rs`。

```rust
use crate::kubectl::KubectlClient;

let client = KubectlClient::new();
```

## 对外接口

### 资源获取方法

- `get_namespaces()` - 获取所有命名空间
- `get_pods(namespace)` - 获取指定命名空间的 Pod 列表
- `get_services(namespace)` - 获取指定命名空间的 Service 列表
- `get_deployments(namespace)` - 获取指定命名空间的 Deployment 列表
- `get_jobs(namespace)` - 获取指定命名空间的 Job 列表
- `get_daemonsets(namespace)` - 获取指定命名空间的 DaemonSet 列表
- `get_pvcs(namespace)` - 获取指定命名空间的 PVC 列表
- `get_pvs()` - 获取所有 PV 列表
- `get_nodes()` - 获取所有 Node 列表
- `get_configmaps(namespace)` - 获取指定命名空间的 ConfigMap 列表
- `get_secrets(namespace)` - 获取指定命名空间的 Secret 列表

### 日志和描述方法

- `get_pod_logs(namespace, pod_name, lines)` - 获取 Pod 日志
- `describe_pod(namespace, pod_name)` - 获取 Pod 详细描述
- `describe_service(namespace, service_name)` - 获取 Service 详细描述
- `describe_deployment(namespace, deployment_name)` - 获取 Deployment 详细描述
- `describe_job(namespace, job_name)` - 获取 Job 详细描述
- `describe_daemonset(namespace, daemonset_name)` - 获取 DaemonSet 详细描述
- `describe_node(node_name)` - 获取 Node 详细描述
- `describe_configmap(namespace, configmap_name)` - 获取 ConfigMap 详细描述
- `describe_secret(namespace, secret_name)` - 获取 Secret 详细描述
- `describe_pvc(namespace, pvc_name)` - 获取 PVC 详细描述
- `describe_pv(pv_name)` - 获取 PV 详细描述

### YAML 获取方法

- `get_yaml(resource_type, namespace, name)` - 获取资源的 YAML 配置

### 指标获取方法

- `get_pod_metrics(namespace)` - 获取 Pod 资源使用指标

## 关键依赖与配置

- **serde_json** - JSON 解析
- **anyhow** - 错误处理
- **tokio** - 异步运行时
- **chrono** - 时间处理
- **外部依赖**: 需要系统安装 kubectl 命令

## 数据模型

主要类型定义在 `src/kubectl/types.rs` 中，包括:

- `Pod` - Pod 资源
- `Service` - Service 资源
- `Namespace` - 命名空间
- `Node` - Node 资源
- `ConfigMap` - ConfigMap 资源
- `Secret` - Secret 资源
- `Deployment` - Deployment 资源
- `Job` - Job 资源
- `DaemonSet` - DaemonSet 资源
- `PVC` - 持久卷声明
- `PV` - 持久卷
- `ResourceMetrics` - 资源使用指标

## 测试与质量

- 单元测试位置: 在各模块文件中
- 使用方法: `cargo test`
- 主要测试内容: 命令构建、响应解析

## 常见问题 (FAQ)

### 如何处理 kubectl 未找到的错误？

`KubectlClient::check_available()` 方法会检查 kubectl 是否可用，返回布尔值。

### 如何自定义资源解析逻辑？

在 `src/kubectl/client.rs` 中修改 parse_* 方法。

## 相关文件清单

| 文件路径 | 描述 |
|---------|------|
| src/kubectl/mod.rs | 模块导出定义 |
| src/kubectl/client.rs | 客户端实现 |
| src/kubectl/commands.rs | 命令构建器 |
| src/kubectl/types.rs | 类型定义 |

## 变更记录 (Changelog)

- **2025-11-01**: 初始化文档
