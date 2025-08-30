#!/bin/bash

# 测试脚本 - 验证鼠标滚轮功能

echo "=== Kube TUI 鼠标滚轮测试 ==="
echo ""
echo "测试步骤："
echo "1. 启动应用"
echo "2. 选择一个Pod并按Space查看详情(Describe模式)"
echo "3. 按Y键查看YAML配置"
echo "4. 尝试使用鼠标滚轮滚动"
echo "5. 使用Esc键返回并测试其他模式"
echo ""
echo "预期结果："
echo "- 在Describe模式和YamlView模式下，鼠标滚轮应该可以滚动内容"
echo "- J/K键盘快捷键应该仍然正常工作"
echo "- 不应影响文本选择功能"
echo ""

read -p "按Enter键启动测试..."

cd /Users/zhanglianwei/github/kube
cargo run

echo ""
echo "测试完成！"