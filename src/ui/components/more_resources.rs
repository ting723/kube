use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // 标题说明
            Constraint::Min(0),    // 资源列表
        ])
        .split(area);

    // 标题说明
    let title_text = if app.language_chinese {
        "更多资源 - 使用数字键 1-7 快速访问资源类型"
    } else {
        "More Resources - Use number keys 1-7 to quickly access resource types"
    };
    
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(title, chunks[0]);

    // 资源列表项
    let items = if app.language_chinese {
        vec![
            ListItem::new("1. PVCs - 持久化存储声明"),
            ListItem::new("2. PVs - 持久化存储卷"),
            ListItem::new("3. Nodes - 节点管理"),
            ListItem::new("4. ConfigMaps - 配置管理"),
            ListItem::new("5. Secrets - 密钥管理"),
            ListItem::new("6. Jobs - 任务管理"),
            ListItem::new("7. DaemonSets - 守护进程集"),
        ]
    } else {
        vec![
            ListItem::new("1. PVCs - Persistent Volume Claims"),
            ListItem::new("2. PVs - Persistent Volumes"),
            ListItem::new("3. Nodes - Node Management"),
            ListItem::new("4. ConfigMaps - Configuration Management"),
            ListItem::new("5. Secrets - Secret Management"),
            ListItem::new("6. Jobs - Job Management"),
            ListItem::new("7. DaemonSets - DaemonSet Management"),
        ]
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(
            if app.language_chinese { 
                "更多资源类型" 
            } else { 
                "More Resource Types" 
            }
        ))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    state.select(Some(app.selected_more_resource_index));

    f.render_stateful_widget(list, chunks[1], &mut state);
}