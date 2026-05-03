use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::state::AppState;

pub fn render(f: &mut Frame, _area: Rect, _app: &AppState) {
    let help_text = r#"
Kube TUI - Kubernetes Terminal Interface v0.2.0
Kube TUI - Kubernetes 终端界面工具 v0.2.0

KEY BINDINGS / 快捷键:
  q                  Quit / 退出
  ?                  Help / 帮助
  I                  Toggle language (Chinese/English) / 切换语言

NAVIGATION / 导航:
  j/k  or  ↑/↓       Navigate lists / 列表导航
  h/l  or  ←/→       Switch panels / 切换面板
  Tab                 Next panel / 下一个面板
  Shift+Tab           Previous panel / 上一个面板
  Enter               Select namespace / Describe resource / 选择/查看详情
  Esc                 Back / Cancel / 返回/取消
  >                   Cycle sort column / 循环切换排序列

RESOURCE ACTIONS / 资源操作:
  Space               Describe resource / 查看资源详情
  Y                   View YAML configuration / 查看 YAML 配置
  T                   View resource metrics (CPU/Memory) / 查看资源监控
  L                   View pod logs / 查看 Pod 日志
  D                   Delete resource (confirmation required) / 删除资源
  E                   Exec into pod container / 进入容器
  /                   Search resources / 搜索资源
  n/N                 Next/Previous search result / 下一个/上一个搜索结果

BATCH OPERATIONS / 批量操作 (列表模式下):
  v                   Toggle batch mode / 切换批量模式
  Space               Mark/unmark item (marked items show ✓ green) / 标记/取消标记
  Ctrl+A              Select all items / 全选
  d                   Delete marked items / 删除已标记项
  Esc                 Exit batch mode / 退出批量模式

LOGS VIEW / 日志视图:
  j/k                 Scroll line by line / 逐行滚动
  PgUp/PgDn           Scroll page by page / 翻页滚动
  V                   Split pane / Add log pane / 分屏/追加日志窗格
  W                   Close focused pane / 关闭当前窗格
  Tab                 Switch focus between panes / 切换窗格焦点
  /                   Search within logs / 日志内搜索
  n/N                 Next/Previous log search match / 下/上一个匹配
  Enter               Confirm search, keep navigating / 确认搜索
  Esc                 Clear search / exit / 清除搜索/退出
  A                   Toggle auto-scroll / 切换自动滚动
  R                   Toggle auto-refresh / 切换自动刷新
  M                   Toggle mouse mode / 切换鼠标模式

CLUSTER & NETWORK / 集群与网络:
  C                   Switch Kubernetes context / 切换集群 Context
  P                   Start port-forward (PodList mode) / 启动端口转发
  Ctrl+P              Stop all port-forwards / 停止所有端口转发

REFRESH STATUS / 刷新状态:
  Header shows [Refresh:ON]/[Refresh:OFF] / 标题栏显示刷新状态
  Auto-refresh every 5 seconds / 每5秒自动刷新
  R (Logs mode)       Toggle log auto-refresh / 切换日志自动刷新
  R (Describe/YAML)   Toggle describe/yaml auto-refresh / 切换描述/YAML自动刷新
  R (list modes)      Manual refresh / 手动刷新
  Log pane shows [AutoScroll:ON/OFF] / 日志窗格显示自动滚动状态

STATUS COLORS / 状态颜色:
  Green=Running / 绿色=运行中    Yellow=Pending / 黄色=等待中
  Red=Failed / 红色=失败         Blue=Succeeded / 蓝色=已完成
  Logs: Red=ERROR lines / 红色=错误    Yellow=WARN lines / 黄色=警告

CONFIGURATION / 配置:
  ~/.config/kube-tui/keys.json    Custom keybindings / 自定义快捷键
  ~/.config/kube-tui/config.json  App configuration / 应用配置

NOTE / 注意:
  • metrics-server required for Top view (T key)
  • 监控视图 (T键) 需要安装 metrics-server
  • Port-forward runs in background (Ctrl+P to stop)
  • 端口转发后台运行 (Ctrl+P 停止)
  • Batch mode works for Pod/Service/Deployment/Job/DaemonSet/ConfigMap/Secret/PVC/PV
  • 批量模式支持 Pod/Service/Deployment/Job/DaemonSet/ConfigMap/Secret/PVC/PV
"#;

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help / 帮助 (? to close / 关闭)"),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, _area);
}
