#!/bin/bash
# Claude Code 配置迁移脚本
# 作者: 猫娘工程师 幽浮喵
# 日期: 2026-04-27

set -e

echo "🚀 开始迁移 Claude Code 配置..."
echo ""

# ==================== Step 1: 安装 Claude Code ====================
echo "📦 Step 1: 安装 Claude Code"
echo "请在新电脑上先安装 Claude Code:"
echo "  npm install -g @anthropic-ai/claude-code"
echo "或者参考官方文档: https://docs.anthropic.com/claude-code"
echo ""
read -p "是否已安装 Claude Code? (y/n): " installed
if [[ "$installed" != "y" ]]; then
    echo "请先安装 Claude Code后再继续"
    exit 1
fi

# ==================== Step 2: 创建目录结构 ====================
echo "📁 Step 2: 创建目录结构"
mkdir -p ~/.claude/rules
mkdir -p ~/.claude/plugins/cache
mkdir -p ~/.claude/commands
echo "✅ 目录结构创建完成"
echo ""

# ==================== Step 3: 复制配置文件 ====================
echo "📄 Step 3: 复制配置文件"
echo "请手动复制以下文件到新电脑:"
echo ""
echo "必需文件:"
echo "  ~/.claude.json                    -> ~/.claude.json"
echo "  ~/.claude/settings.json           -> ~/.claude/settings.json"
echo ""
echo "Rules 配置 (9个文件):"
echo "  ~/.claude/rules/*.md              -> ~/.claude/rules/"
echo ""
read -p "是否已复制配置文件? (y/n): " copied
if [[ "$copied" != "y" ]]; then
    echo "请先复制配置文件后再继续"
    exit 1
fi

# ==================== Step 4: 安装依赖 ====================
echo "📦 Step 4: 检查并安装依赖"
echo ""

# 检查 Node.js
if command -v node &> /dev/null; then
    echo "✅ Node.js 已安装: $(node --version)"
else
    echo "❌ Node.js 未安装，请先安装 Node.js"
    exit 1
fi

# 检查 npx
if command -v npx &> /dev/null; then
    echo "✅ npx 已安装"
else
    echo "❌ npx 未安装，请先安装 npm"
    exit 1
fi

# 检查 uvx (Python 包管理器，serena 依赖)
if command -v uvx &> /dev/null; then
    echo "✅ uvx 已安装"
else
    echo "⚠️  uvx 未安装，serena MCP server 将无法使用"
    echo "   安装方法: pip install uvx 或参考 https://docs.astral.sh/uv/"
fi

echo ""

# ==================== Step 5: 安装 Plugins ====================
echo "🔌 Step 5: 安装 Plugins"
echo ""
echo "请在 Claude Code 中执行以下命令来安装 Plugins:"
echo ""
echo "1. 安装 everything-claude-code:"
echo "   /plugin:install everything-claude-code"
echo ""
echo "2. 安装 claude-hud:"
echo "   /plugin:install claude-hud"
echo ""
echo "3. 安装 superpowers-marketplace:"
echo "   /plugin:install superpowers-marketplace"
echo ""
echo "4. 启用 superpowers-dev 和 superpowers:"
echo "   在 settings.json 的 enabledPlugins 中设置:"
echo "   \"superpowers-dev@superpowers-marketplace\": true"
echo "   \"superpowers@superpowers-marketplace\": true"
echo ""
read -p "是否已安装 Plugins? (y/n): " plugins_installed
if [[ "$plugins_installed" != "y" ]]; then
    echo "请先安装 Plugins 后再继续"
    exit 1
fi

# ==================== Step 6: 验证 MCP Servers ====================
echo "🔌 Step 6: 验证 MCP Servers"
echo ""
echo "启动 Claude Code 后，MCP servers 会自动安装。"
echo "请验证以下 MCP servers 是否正常运行:"
echo ""
echo "  context7        - 文档查询"
echo "  open-websearch  - 网络搜索"
echo "  spec-workflow   - 规范工作流"
echo "  mcp-deepwiki    - DeepWiki 文档"
echo "  Playwright      - 浏览器自动化"
echo "  exa             - Exa 搜索 (需要配置 EXA_API_KEY)"
echo "  serena          - IDE 辅助"
echo ""
echo "验证命令:"
echo "  在 Claude Code 中输入: /mcp"
echo ""
read -p "是否已验证 MCP Servers? (y/n): " mcp_verified

# ==================== Step 7: 配置 API Token ====================
echo "🔑 Step 7: 配置 API Token"
echo ""
echo "请确保以下 API Token 已正确配置:"
echo ""
echo "1. ANTHROPIC_AUTH_TOKEN (在 ~/.claude/settings.json 的 env 中)"
echo "2. EXA_API_KEY (在 ~/.claude.json 的 mcpServers.exa.env 中)"
echo ""
read -p "是否已配置 API Tokens? (y/n): " tokens_configured

# ==================== Step 8: 完成迁移 ====================
echo ""
echo "🎉 配置迁移完成!"
echo ""
echo "后续步骤:"
echo "1. 启动 Claude Code: claude"
echo "2. 验证主题和输出风格: /config"
echo "3. 验证 MCP servers: /mcp"
echo "4. 验证 Plugins: /plugins"
echo ""
echo "如有问题，请参考:"
echo "  ~/.claude/settings.json  - 全局设置"
echo "  ~/.claude.json           - MCP servers 和项目记录"
echo "  ~/.claude/rules/         - 开发规则"
echo ""
echo "祝您使用愉快喵～ ≡ω≡"