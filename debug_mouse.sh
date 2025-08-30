#!/bin/bash

echo "=== 鼠标滚轮功能调试测试 ==="
echo ""
echo "此脚本将启动应用并记录鼠标事件到debug.log文件"
echo ""

# 清理之前的调试日志
rm -f /Users/zhanglianwei/github/kube/debug.log

# 启动应用
cd /Users/zhanglianwei/github/kube
./target/debug/kube-tui

echo ""
echo "应用已退出。检查debug.log文件查看鼠标事件记录："
echo ""
if [ -f debug.log ]; then
    echo "=== 调试日志内容 ==="
    cat debug.log
else
    echo "没有找到调试日志文件"
fi