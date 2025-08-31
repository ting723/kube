#!/bin/bash

echo "=== 鼠标功能测试 ==="
echo ""
echo "请测试以下功能："
echo "1. 在 Describe/YAML/Log 模式下使用鼠标滚轮滚动"
echo "2. 在任何模式下尝试选中并复制文本"
echo ""
echo "预期结果："
echo "- 鼠标滚轮应该可以正常滚动内容"
echo "- 应该可以选中并复制文本（使用鼠标拖拽选择）"
echo ""

cd /Users/zhanglianwei/github/kube
cargo run

echo ""
echo "测试完成！"
echo "如果发现问题，请报告具体的行为："
echo "- 鼠标滚轮是否工作？"
echo "- 文本选择是否工作？"