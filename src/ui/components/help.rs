use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, _area: Rect, _app: &AppState) {
    let help_text = if _app.language_chinese {
        r#"
Kube TUI - Kubernetes Terminal Interface
Kube TUI - Kubernetes 终端界面工具

NAVIGATION / 导航:
  j/k or ↑/↓       Navigate lists / 列表导航
  h/l or ←/→       Switch panels / 切换面板
  Tab              Switch to next panel / 切换到下一个面板
  Shift+Tab        Switch to previous panel / 切换到上一个面板
  Enter            Select item / 选择项目
  Esc              Go back / 返回上级
  q                Quit application / 退出程序
  ?                Show this help / 显示帮助

PANEL ACCESS / 面板访问:
  Tab              循环切换核心面板 (Namespaces → Pods → Services → Deployments)
  Shift+Tab        反向循环切换核心面板
  F1               Nodes面板
  F2               ConfigMaps面板
  F3               Secrets面板
  F4               Jobs面板
  F5               DaemonSets面板
  F6               More Resources面板
  F7               Help面板

NAMESPACE VIEW / 命名空间视图:
  Enter            Switch to selected namespace / 切换到选中的命名空间

POD VIEW / Pod 视图:
  Space            Describe pod / 查看 Pod 详情
  Y                View YAML config / 查看 YAML 配置
  T                View resource usage / 查看资源使用情况
  L                View pod logs / 查看 Pod 日志
  D                Delete pod / 删除 Pod（需确认）
  E                Exec into pod / 进入 Pod 容器
  /                Search pods / 搜索 Pod

LOGS VIEW / 日志视图:
  J/K              Scroll line by line / 按行滚动
  PgUp/PgDn        Scroll page by page / 按页滚动
  A                Toggle auto-scroll / 切换自动滚动
  R                Toggle auto-refresh / 切换自动刷新
  Esc              Return to pod list / 返回 Pod 列表

YAML/DESCRIBE/TOP VIEW / YAML/描述/监控视图:
  J/K              Scroll content / 滚动内容
  PgUp/PgDn        Scroll page by page / 按页滚动
  Esc              Return to previous view / 返回上一级视图

MORE RESOURCES PANEL / 更多资源面板:
  1                PVCs - 持久化存储声明
  2                PVs - 持久化存储卷
  3                Nodes - 节点管理
  4                ConfigMaps - 配置管理
  5                Secrets - 密钥管理
  6                Jobs - 任务管理
  7                DaemonSets - 守护进程集
  Esc              Return to main panels / 返回主面板

SERVICE/DEPLOYMENT VIEW / 服务/部署视图:
  Space            Describe resource / 查看资源详情
  Y                View YAML config / 查看 YAML 配置
  /                Search resources / 搜索资源

RESOURCE MONITORING / 资源监控:
  T (in Pod view)  View CPU/Memory usage / 查看 CPU/内存使用情况
  Note: Requires metrics-server / 注意: 需要安装 metrics-server

SEARCH / 搜索:
  /                Start search / 开始搜索
  Type query       Enter search terms / 输入搜索内容
  ↑/↓              Navigate search results / 导航搜索结果
  Enter            Confirm search / 确认搜索
  n/N              Next/Previous result / 下一个/上一个结果
  Esc              Cancel search / 取消搜索

CONFIRM DIALOG / 确认对话框:
  y/Y              Confirm action / 确认操作
  n/N/Esc          Cancel action / 取消操作

GENERAL / 常规:
  • Auto-refresh every 5 seconds / 每5秒自动刷新
  • Status colors: Green=Running, Yellow=Pending, Red=Failed
  • 状态颜色: 绿色=运行中, 黄色=等待中, 红色=失败
  • YAML syntax highlighting / YAML 语法高亮
  • Mouse text selection supported / 支持鼠标文字选择
  • Command line shows current kubectl operations
  • 命令行显示当前 kubectl 操作
"#
    } else {
        r#"
Kube TUI - Kubernetes Terminal Interface

NAVIGATION:
  j/k or ↑/↓       Navigate lists
  h/l or ←/→       Switch panels
  Tab              Switch to next panel
  Shift+Tab        Switch to previous panel
  Enter            Select item
  Esc              Go back
  q                Quit application
  ?                Show this help

PANEL ACCESS:
  Tab              Cycle through core panels (Namespaces → Pods → Services → Deployments)
  Shift+Tab        Cycle through core panels in reverse
  F1               Nodes panel
  F2               ConfigMaps panel
  F3               Secrets panel
  F4               Jobs panel
  F5               DaemonSets panel
  F6               More Resources panel
  F7               Help panel

NAMESPACE VIEW:
  Enter            Switch to selected namespace

POD VIEW:
  Space            Describe pod
  Y                View YAML config
  T                View resource usage
  L                View pod logs
  D                Delete pod (requires confirmation)
  E                Exec into pod
  /                Search pods

LOGS VIEW:
  J/K              Scroll line by line
  PgUp/PgDn        Scroll page by page
  A                Toggle auto-scroll
  R                Toggle auto-refresh
  Esc              Return to pod list

YAML/DESCRIBE/TOP VIEW:
  J/K              Scroll content
  PgUp/PgDn        Scroll page by page
  Esc              Return to previous view

MORE RESOURCES PANEL:
  1                PVCs - Persistent Volume Claims
  2                PVs - Persistent Volumes
  3                Nodes - Node Management
  4                ConfigMaps - Configuration Management
  5                Secrets - Secret Management
  6                Jobs - Job Management
  7                DaemonSets - DaemonSet Management
  Esc              Return to main panels

SERVICE/DEPLOYMENT VIEW:
  Space            Describe resource
  Y                View YAML config
  /                Search resources

RESOURCE MONITORING:
  T (in Pod view)  View CPU/Memory usage
  Note: Requires metrics-server

SEARCH:
  /                Start search
  Type query       Enter search terms
  ↑/↓              Navigate search results
  Enter            Confirm search
  n/N              Next/Previous result
  Esc              Cancel search

CONFIRM DIALOG:
  y/Y              Confirm action
  n/N/Esc          Cancel action

GENERAL:
  • Auto-refresh every 5 seconds
  • Status colors: Green=Running, Yellow=Pending, Red=Failed
  • YAML syntax highlighting
  • Mouse text selection supported
  • Command line shows current kubectl operations
"#
    };

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if _app.language_chinese { "Help / 帮助" } else { "Help" })
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, _area);
}