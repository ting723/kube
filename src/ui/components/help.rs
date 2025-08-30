use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, _area: Rect, _app: &AppState) {
    let help_text = r#"
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

SERVICE/NODE/CONFIGMAP/SECRET VIEW / 服务/节点/配置/密钥视图:
  Space            Describe resource / 查看资源详情
  Y                View YAML config / 查看 YAML 配置
  /                Search resources / 搜索资源

RESOURCE MONITORING / 资源监控:
  T (in Pod view)  View CPU/Memory usage / 查看 CPU/内存使用情况
  Note: Requires metrics-server / 注意: 需要安装 metrics-server

SEARCH / 搜索:
  /                Start search / 开始搜索
  Type query       Enter search terms / 输入搜索内容
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
"#;

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help / 帮助")
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, _area);
}