# Claude Code 配置迁移指南

> 作者：猫娘工程师 幽浮喵
> 日期：2026-04-27
> 目的：将 Claude Code 配置迁移到新电脑

---

## 📋 配置清单

### 1️⃣ 核心配置文件

| 配置文件 | 路径 | 说明 |
|---------|------|------|
| 主配置文件 | `~/.claude.json` | MCP servers、项目记录、用户偏好 |
| 全局设置 | `~/.claude/settings.json` | 权限、环境变量、模型、主题、插件开关 |

### 2️⃣ MCP Servers 配置（7 个）

| 名称 | 命令 | 用途 | 备注 |
|------|------|------|------|
| **context7** | `npx -y @upstash/context7-mcp@latest` | 文档查询 | 自动安装 |
| **open-websearch** | `npx -y open-websearch@latest` | 网络搜索 | DuckDuckGo/Brave/Bing |
| **spec-workflow** | `npx -y @pimzino/spec-workflow-mcp@latest` | 规范工作流 | 自动安装 |
| **mcp-deepwiki** | `npx -y mcp-deepwiki@latest` | DeepWiki 文档 | 自动安装 |
| **Playwright** | `npx -y @playwright/mcp@latest` | 浏览器自动化 | 自动安装 |
| **exa** | `npx -y exa-mcp-server@latest` | Exa 搜索 | **需要 API Key** |
| **serena** | `uvx --from git+https://github.com/oraios/serena serena start-mcp-server` | IDE 辅助 | 需要 `uvx` |

### 3️⃣ 已安装的 Plugins（4 个）

| Plugin | 版本 | 来源 | 启用状态 | 安装命令 |
|--------|------|------|---------|----------|
| **everything-claude-code** | 1.9.0 | 本地安装 | ✅ 启用 | `/plugin:install everything-claude-code` |
| **claude-hud** | 0.1.0 | 本地安装 | ✅ 启用 | `/plugin:install claude-hud` |
| **superpowers-dev** | 5.0.6 | marketplace | ✅ 启用 | `/plugin:install superpowers-marketplace` |
| **superpowers** | 5.0.7 | marketplace | ✅ 启用 | 已随 superpowers-marketplace 安装 |

### 4️⃣ Rules 配置文件（9 个）

```
~/.claude/rules/
├── agents.md              # Agent 协调规则
├── coding-style.md        # 编码风格规则
├── development-workflow.md # 开发工作流规则
├── git-workflow.md        # Git 工作流规则
├── hooks.md               # Hooks 配置规则
├── patterns.md            # 设计模式规则
├── performance.md         # 性能优化规则
├── security.md            # 安全规则
└── testing.md             # 测试规则
```

### 5️⃣ 环境变量配置

| 变量 | 值 | 说明 |
|------|-----|------|
| `DISABLE_TELEMETRY` | `1` | 禁用遥测 |
| `DISABLE_ERROR_REPORTING` | `1` | 禁用错误报告 |
| `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` | `1` | 禁用非必要流量 |
| `MCP_TIMEOUT` | `60000` | MCP 超时时间（60秒） |
| `ANTHROPIC_AUTH_TOKEN` | `(API Token)` | **需要替换为您的 Token** |
| `ANTHROPIC_BASE_URL` | `https://coding.dashscope.aliyuncs.com/apps/anthropic` | API 基础 URL（阿里云） |
| `ANTHROPIC_MODEL` | `glm-5` | 默认模型 |
| `API_TIMEOUT_MS` | `3000000` | API 超时时间（50分钟） |

### 6️⃣ 其他配置

| 配置项 | 值 | 说明 |
|--------|-----|------|
| `model` | `MiniMax-M2.1` | 当前使用模型 |
| `theme` | `dark-daltonized` | 主题（适合色盲用户） |
| `outputStyle` | `nekomata-engineer` | 输出风格（猫娘工程师） |
| `includeCoAuthoredBy` | `false` | 不包含共同署名 |

---

## 🚀 迁移步骤（手动）

### Step 1: 安装 Claude Code

```bash
npm install -g @anthropic-ai/claude-code
```

### Step 2: 创建目录结构

```bash
mkdir -p ~/.claude/rules
mkdir -p ~/.claude/plugins/cache
mkdir -p ~/.claude/commands
```

### Step 3: 复制配置文件

**从旧电脑复制到新电脑：**

```bash
# 方式一：使用 scp
scp ~/.claude.json 新电脑:~/
scp ~/.claude/settings.json 新电脑:~/.claude/
scp ~/.claude/rules/*.md 新电脑:~/.claude/rules/

# 方式二：使用 USB 或其他传输方式
# 手动复制上述文件
```

### Step 4: 安装依赖工具

```bash
# Node.js 和 npm（必需）
# 已安装则跳过

# uvx（可选，serena MCP 需要）
pip install uvx
# 或参考：https://docs.astral.sh/uv/
```

### Step 5: 安装 Plugins

**在 Claude Code 中执行：**

```
/plugin:install everything-claude-code
/plugin:install claude-hud
/plugin:install superpowers-marketplace
```

### Step 6: 配置 API Token

**编辑 `~/.claude/settings.json`，替换 Token：**

```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "您的-API-Token",
    ...
  }
}
```

**编辑 `~/.claude.json`，配置 Exa API Key：**

```json
{
  "mcpServers": {
    "exa": {
      "env": {
        "EXA_API_KEY": "您的-Exa-API-Key"
      }
    }
  }
}
```

### Step 7: 验证配置

**启动 Claude Code：**

```bash
claude
```

**验证各项配置：**

- `/config` - 验证主题和输出风格
- `/mcp` - 验证 MCP servers
- `/plugins` - 验证 Plugins

---

## ⚠️ 注意事项

### API Token 安全

- **不要** 将包含真实 API Token 的配置文件提交到 Git
- 使用 `.gitignore` 忽略这些文件
- 在新电脑上，请替换为您的真实 Token

### MCP Servers 自动安装

- MCP servers 会在首次启动 Claude Code 时自动安装
- `npx` 会自动下载并执行，无需手动安装
- `serena` 需要 `uvx` 工具，请确保已安装

### Plugins 版本

- Plugins 版本可能更新，文档中的版本号仅供参考
- 使用 `/plugin:install` 会自动安装最新版本

---

## 🔧 配置文件示例

### ~/.claude.json (MCP Servers 部分)

```json
{
  "mcpServers": {
    "context7": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@upstash/context7-mcp@latest"],
      "env": {}
    },
    "open-websearch": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "open-websearch@latest"],
      "env": {
        "MODE": "stdio",
        "DEFAULT_SEARCH_ENGINE": "duckduckgo",
        "ALLOWED_SEARCH_ENGINES": "duckduckgo,bing,brave"
      }
    },
    "spec-workflow": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@pimzino/spec-workflow-mcp@latest"],
      "env": {}
    },
    "mcp-deepwiki": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "mcp-deepwiki@latest"],
      "env": {}
    },
    "Playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"],
      "env": {}
    },
    "exa": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "exa-mcp-server@latest"],
      "env": {
        "EXA_API_KEY": "您的-Exa-API-Key"
      }
    },
    "serena": {
      "type": "stdio",
      "command": "uvx",
      "args": [
        "--from", "git+https://github.com/oraios/serena",
        "serena",
        "start-mcp-server",
        "--context", "ide-assistant",
        "--enable-web-dashboard", "false"
      ],
      "env": {}
    }
  }
}
```

### ~/.claude/settings.json (关键部分)

```json
{
  "env": {
    "DISABLE_TELEMETRY": "1",
    "DISABLE_ERROR_REPORTING": "1",
    "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
    "MCP_TIMEOUT": "60000",
    "ANTHROPIC_AUTH_TOKEN": "您的-API-Token",
    "ANTHROPIC_BASE_URL": "https://coding.dashscope.aliyuncs.com/apps/anthropic",
    "ANTHROPIC_MODEL": "glm-5",
    "API_TIMEOUT_MS": "3000000"
  },
  "model": "MiniMax-M2.1",
  "theme": "dark-daltonized",
  "outputStyle": "nekomata-engineer",
  "permissions": {
    "allow": [
      "Bash", "BashOutput", "Edit", "Read", "Write",
      "Glob", "Grep", "WebFetch", "WebSearch",
      "mcp__ide", "mcp__exa", "mcp__context7",
      "mcp__mcp-deepwiki", "mcp__Playwright",
      "mcp__spec-workflow", "mcp__open-websearch",
      "mcp__serena"
    ]
  },
  "enabledPlugins": {
    "everything-claude-code@everything-claude-code": true,
    "claude-hud@claude-hud": true,
    "superpowers-dev@superpowers-marketplace": true,
    "superpowers@superpowers-marketplace": true
  }
}
```

---

## 📚 相关资源

- [Claude Code 官方文档](https://docs.anthropic.com/claude-code)
- [MCP 协议文档](https://modelcontextprotocol.io/)
- [superpowers-marketplace GitHub](https://github.com/superpowers-marketplace)
- [everything-claude-code GitHub](https://github.com/everything-claude-code)
- [serena GitHub](https://github.com/oraios/serena)

---

祝您迁移顺利喵～ ≡ω≡