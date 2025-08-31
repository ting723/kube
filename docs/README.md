# Kube TUI 文档中心

## 📚 文档目录

### 🚀 用户指南
- [README.md](../README.md) - 项目主要说明文档
- [USAGE.md](../USAGE.md) - 详细使用指南

### 📖 功能指南
- [双模式切换指南](guides/DUAL_MODE_GUIDE.md) - YAML/Describe模式的文本选择与鼠标滚轮切换
- [Minikube支持说明](guides/MINIKUBE_SUPPORT.md) - minikube环境下的kubectl支持
- [优化功能指南](guides/OPTIMIZATIONS_GUIDE.md) - 各种UI和功能优化说明

### 🔧 开发与维护
- [修复记录](fixes/) - 历史修复和优化记录归档
- [测试脚本](../scripts/) - 各种功能测试脚本

## 🎯 快速导航

### 新用户入门
1. 查看 [README.md](../README.md) 了解项目概览
2. 阅读 [USAGE.md](../USAGE.md) 学习使用方法
3. 运行 `scripts/test_optimizations.sh` 体验所有功能

### 功能测试
```bash
# 测试核心功能
./scripts/test_optimizations.sh

# 测试minikube支持
./scripts/test_minikube_support.sh

# 测试双模式切换
./scripts/test_dual_mode_switching.sh
```

### 开发者指南
- 构建项目：`./scripts/build.sh`
- 查看修复历史：`docs/fixes/README.md`
- 了解项目规范：参考memory中的项目规范

## 📋 项目结构
```
kube/
├── src/                    # 源代码
├── docs/                   # 文档中心
│   ├── guides/            # 功能指南
│   └── fixes/             # 修复记录归档
├── scripts/               # 脚本集合
│   ├── build.sh          # 构建脚本
│   └── test_*.sh         # 测试脚本
├── README.md              # 项目主文档
├── USAGE.md               # 使用指南
├── Cargo.toml             # Rust项目配置
└── Makefile               # 构建配置
```

这个结构化的文档中心让您可以快速找到所需的信息和工具！