#!/bin/bash

# Kube TUI 构建脚本

set -e

echo "🔧 开始构建 Kube TUI..."

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 没有找到 cargo。请先安装 Rust。"
    echo "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 检查 kubectl 是否安装
if ! command -v kubectl &> /dev/null; then
    echo "⚠️  警告: 没有找到 kubectl。应用运行时需要 kubectl。"
    echo "   请确保已安装 kubectl 并配置好 kubeconfig。"
fi

echo "📦 构建发布版本..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ 构建成功!"
    echo "📍 二进制文件位置: target/release/kube-tui"
    echo ""
    echo "🚀 运行应用:"
    echo "   ./target/release/kube-tui"
    echo ""
    echo "📋 或者创建符号链接到 PATH:"
    echo "   sudo ln -sf $(pwd)/target/release/kube-tui /usr/local/bin/kube-tui"
    echo ""
else
    echo "❌ 构建失败!"
    exit 1
fi