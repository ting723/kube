#!/bin/bash

# kubectl 检查测试脚本

echo "=== kubectl 可用性测试 ==="

echo "1. 测试 kubectl version --client:"
kubectl version --client 2>/dev/null
echo "退出状态: $?"
echo ""

echo "2. 测试 kubectl version:"
kubectl version 2>/dev/null | head -5
echo "退出状态: $?"
echo ""

echo "3. 测试 kubectl --help:"
kubectl --help 2>/dev/null | head -3
echo "退出状态: $?"
echo ""

echo "4. 测试 kubectl cluster-info:"
kubectl cluster-info 2>/dev/null
echo "退出状态: $?"
echo ""

echo "=== kube-tui 启动测试 ==="
echo "如果程序正常启动，请按 q 退出"
echo ""

# 启动 kube-tui（超时 5 秒后自动退出，避免卡住）
timeout 5s ./target/release/kube-tui || echo "程序已自动退出或超时"