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
- 按 `l` 查看日志
- 按 `d` 查看详细描述
- 按 `e` 进入 Pod (需要支持的容器)

### 3. 服务查看
- 按 `Tab` 切换到服务面板
- 查看服务的类型、IP 和端口信息

### 4. 实时更新
- 应用每 5 秒自动刷新数据
- 状态变化会实时反映在界面上

## 快捷键速查

| 键位 | 功能 |
|------|------|
| `↑/↓` | 导航 |
| `Tab` | 切换面板 |
| `Enter` | 选择/进入 |
| `Esc` | 返回/退出 |
| `l` | 查看日志 (Pod 面板) |
| `d` | 描述资源 |
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

## 扩展功能 (未来计划)

- [ ] Pod 删除确认对话框
- [ ] 实时日志流
- [ ] 搜索和过滤功能
- [ ] 配置文件支持
- [ ] 主题自定义
- [ ] 更多资源类型支持 (Deployments, ConfigMaps, etc.)