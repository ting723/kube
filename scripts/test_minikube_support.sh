#!/bin/bash

echo "=== Minikube kubectl支持功能测试 ==="
echo ""
echo "🎯 功能说明："
echo "本优化为Kube TUI工具添加了智能kubectl命令检测支持，能够："
echo ""
echo "1. 🔍 自动检测kubectl命令类型："
echo "   - 优先尝试直接使用 kubectl 命令"
echo "   - 如果不可用，自动检测并使用 minikube kubectl -- 方式"
echo "   - 使用单例模式缓存检测结果，避免重复检测"
echo ""
echo "2. 🔄 智能命令转换："
echo "   - Direct模式: kubectl get pods"
echo "   - Minikube模式: minikube kubectl -- get pods"
echo "   - 应用层代码无需任何修改，完全透明"
echo ""
echo "3. 🎮 支持所有kubectl操作："
echo "   - 资源查看: get, describe, logs"
echo "   - YAML配置: get -o yaml"
echo "   - 资源操作: delete"
echo "   - 交互式操作: exec（特殊处理，保持终端交互）"
echo ""
echo "4. 📊 监控和错误处理："
echo "   - 提供详细的错误信息和安装指导"
echo "   - 支持两种安装方式的提示"
echo ""
echo "🔧 技术实现："
echo "- KubectlCommand枚举: Direct | Minikube"
echo "- execute_kubectl()抽象函数: 自动选择合适的命令方式"
echo "- check_minikube_kubectl(): 智能检测minikube环境"
echo "- OnceLock单例: 缓存检测结果，提高性能"
echo ""
echo "📋 测试环境检查："

# 检查kubectl是否直接可用
if command -v kubectl >/dev/null 2>&1; then
    echo "✅ kubectl命令直接可用"
    kubectl version --client --short 2>/dev/null || echo "⚠️  kubectl可执行但无法连接集群"
else
    echo "❌ kubectl命令不可用"
fi

# 检查minikube是否可用
if command -v minikube >/dev/null 2>&1; then
    echo "✅ minikube命令可用"
    minikube status 2>/dev/null || echo "⚠️  minikube未启动或未配置"
    
    # 测试minikube kubectl
    if minikube kubectl -- version --client >/dev/null 2>&1; then
        echo "✅ minikube kubectl功能正常"
    else
        echo "❌ minikube kubectl功能异常"
    fi
else
    echo "❌ minikube命令不可用"
fi

echo ""
echo "📋 建议测试场景："
echo ""
echo "场景1: 标准kubectl环境"
echo "- 确保系统已安装kubectl"
echo "- 启动应用，验证所有功能正常"
echo ""
echo "场景2: 纯minikube环境"
echo "- 移除或重命名kubectl（模拟minikube-only环境）"
echo "- 确保minikube正在运行"
echo "- 启动应用，验证自动切换到minikube kubectl模式"
echo ""
echo "场景3: 混合环境"
echo "- 同时安装kubectl和minikube"
echo "- 验证优先使用kubectl，minikube作为后备"
echo ""

read -p "按Enter键启动Kube TUI进行实际测试..."

echo ""
echo "🚀 启动Kube TUI..."
echo "请在应用中测试以下功能："
echo "- 查看namespaces和pods列表"
echo "- 按Space查看Pod详情"
echo "- 按Y查看YAML配置"
echo "- 按L查看日志"
echo "- 验证所有操作在您的环境中都能正常工作"
echo ""

cd /Users/zhanglianwei/github/kube
cargo run

echo ""
echo "=== 测试完成 ==="
echo ""
echo "请报告测试结果："
echo "✓ 应用是否正常启动？"
echo "✓ 能否正常列出namespaces？"
echo "✓ 能否正常查看pods？"
echo "✓ describe和YAML功能是否正常？"
echo "✓ 在您的环境中是否检测到正确的kubectl模式？"
echo ""
echo "🎉 minikube kubectl支持功能已完成！"
echo "现在您的Kube TUI工具可以在以下环境中无缝工作："
echo "• 标准Kubernetes集群 (kubectl)"
echo "• Minikube环境 (minikube kubectl)"
echo "• 混合环境 (智能检测和切换)"