# Kube TUI 脚本集合

## 🔧 构建脚本

### build.sh
项目构建脚本，支持开发和发布构建。

```bash
./scripts/build.sh
```

## 🧪 测试脚本

### 核心功能测试

#### test_optimizations.sh
**综合功能测试脚本** - 测试所有核心优化功能
- 🎯 推荐新用户使用，体验完整功能
- 包含UI优化、交互改进、功能增强等测试

```bash
./scripts/test_optimizations.sh
```

### 专项功能测试

#### test_minikube_support.sh
**Minikube支持功能测试**
- 测试智能kubectl命令检测
- 验证minikube环境兼容性
- 支持环境切换测试

```bash
./scripts/test_minikube_support.sh
```

#### test_dual_mode_switching.sh
**双模式切换功能测试**
- 测试YAML/Describe模式的文本选择模式
- 测试鼠标滚轮模式切换
- 验证M键切换功能

```bash
./scripts/test_dual_mode_switching.sh
```

### 鼠标功能测试

#### test_conditional_mouse_capture.sh
**条件性鼠标捕获测试**
- 测试鼠标滚轮支持
- 验证文本选择兼容性
- 检查模式切换透明性

```bash
./scripts/test_conditional_mouse_capture.sh
```

#### test_mouse.sh
**基础鼠标功能测试**
- 基本鼠标事件处理测试

```bash
./scripts/test_mouse.sh
```

#### test_mouse_selection.sh
**鼠标选择功能测试**
- 文本选择功能验证

```bash
./scripts/test_mouse_selection.sh
```

### 环境测试

#### test_kubectl.sh
**kubectl命令测试**
- 验证kubectl命令可用性
- 检查Kubernetes连接

```bash
./scripts/test_kubectl.sh
```

### 调试工具

#### debug_mouse.sh
**鼠标功能调试脚本**
- 用于调试鼠标相关问题

```bash
./scripts/debug_mouse.sh
```

## 📋 推荐测试流程

### 新用户体验流程
1. `test_optimizations.sh` - 体验核心功能
2. `test_minikube_support.sh` - 测试环境兼容性
3. `test_dual_mode_switching.sh` - 学习高级功能

### 开发测试流程
1. `build.sh` - 构建项目
2. `test_kubectl.sh` - 验证环境
3. 运行相关功能测试脚本

### 问题排查流程
1. `test_kubectl.sh` - 检查基础环境
2. `debug_mouse.sh` - 调试鼠标问题
3. 运行对应的专项测试

## 🎯 脚本使用技巧

所有测试脚本都可以直接运行：
```bash
# 给脚本执行权限（如果需要）
chmod +x scripts/*.sh

# 运行任意测试脚本
./scripts/test_optimizations.sh
```

这些脚本帮助您全面测试和验证Kube TUI的各项功能！