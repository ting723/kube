# 使用示例

## 基本使用

```bash
# 构建项目
./build.sh

# 运行应用
./target/release/kube-tui
```

## 主要功能演示

### 1. 命名空间导航
- 启动后会显示所有可用的命名空间
- 使用 `↑/↓` 键选择命名空间
- 按 `Enter` 进入选中的命名空间

### 2. Pod 管理
- 在 Pod 列表中使用 `↑/↓` 选择 Pod
- 按 `Space` 查看详细描述
- 按 `Y` 查看完整 YAML 配置
- 按 `T` 查看 CPU/内存使用情况
- 按 `L` 查看日志
- 按 `E` 进入 Pod (需要支持的容器)
- 按 `D` 删除 Pod (需要确认)
- 按 `/` 搜索 Pod

### 3. 资源管理
- 按 `Tab` 切换到不同资源面板 (Pods/Services/Deployments/Jobs/PVCs/PVs/Nodes/ConfigMaps/DaemonSets/Secrets)
- 查看资源的类型、IP 和端口信息
- 使用 `Space` 查看详细描述
- 使用 `Y` 查看 YAML 配置

### 4. 日志查看
- 在日志视图中使用 `J/K` 滚动
- 按 `A` 切换自动滚动
- 按 `R` 切换自动刷新
- 支持鼠标滚轮操作

### 5. 资源监控
- 在 Pod 列表中按 `T` 查看资源使用情况
- 显示每个 Pod 的 CPU 和内存使用量
- 显示容器级别的详细指标
- 注意: 需要安装 metrics-server

### 6. YAML 配置查看
- 在任何资源列表中按 `Y` 查看完整配置
- 支持 YAML 语法高亮
- 使用 `J/K` 滚动查看

### 7. 搜索功能
- 在任何列表中按 `/` 开始搜索
- 支持实时搜索和模糊匹配
- 使用 `n/N` 在搜索结果中导航

### 8. 实时更新
- 应用每 5 秒自动刷新数据
- 状态变化会实时反映在界面上

## 快捷键速查

| 键位 | 功能 |
|------|------|
| `↑/↓` | 导航 |
| `j/k` | Vim 风格导航 |
| `Tab` | 切换面板 |
| `Enter` | 选择/进入 |
| `Esc` | 返回/退出 |
| `Space` | 查看详细描述 |
| `Y` | 查看 YAML 配置 |
| `T` | 查看资源使用 (Pod 面板) |
| `L` | 查看日志 (Pod 面板) |
| `E` | 进入容器 (Pod 面板) |
| `D` | 删除资源 |
| `/` | 搜索 |
| `n/N` | 搜索结果导航 |
| `J/K` | 滚动内容 |
| `A` | 切换自动滚动 (日志) |
| `R` | 切换自动刷新 (日志) |
| `?` | 帮助 |
| `q` | 退出应用 |

## 故障排除

### 常见问题

1. **kubectl 未找到**
   ```bash
   # 安装 kubectl (macOS)
   brew install kubectl
   
   # 安装 kubectl (Linux)
   curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
   ```

2. **无法连接集群**
   ```bash
   # 检查集群连接
   kubectl cluster-info
   
   # 检查 kubeconfig
   kubectl config view
   ```

3. **权限不足**
   ```bash
   # 检查当前用户权限
   kubectl auth can-i get pods --all-namespaces
   ```

## 性能优化

- 应用使用异步操作，不会阻塞 UI
- 智能缓存，减少不必要的 API 调用
- 轻量级二进制，启动速度快

## 扩展功能

本版本已实现的功能:
- ✅ Pod 删除确认对话框
- ✅ 实时日志流
- ✅ 搜索和过滤功能
- ✅ YAML 语法高亮
- ✅ 资源监控 (CPU/内存)
- ✅ 更多资源类型支持 (Deployments, ConfigMaps, Jobs, DaemonSets, PVCs, PVs, Secrets)
- ✅ 鼠标支持和滚轮操作

未来计划:
- [ ] 配置文件支持
- [ ] 主题自定义
- [ ] 多集群支持
- [ ] 性能优化和缓存