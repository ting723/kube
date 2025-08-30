use ratatui::{
    layout::{Rect, Constraint},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, Cell},
    Frame,
};

use crate::app::AppState;

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.jobs.is_empty() {
        let no_jobs = ratatui::widgets::Paragraph::new("No jobs found or loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Jobs")
            )
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(no_jobs, area);
        return;
    }

    let rows: Vec<Row> = app
        .jobs
        .iter()
        .enumerate()
        .map(|(i, job)| {
            let style = if i == app.selected_job_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status_color = match job.status.as_str() {
                "Complete" => Color::Green,
                "Running" => Color::Blue,
                "Failed" => Color::Red,
                _ => Color::Gray,
            };

            let completions = job.completions
                .map(|c| c.to_string())
                .unwrap_or_else(|| "1".to_string());
            let duration = job.duration
                .as_ref()
                .unwrap_or(&"<none>".to_string())
                .clone();

            Row::new(vec![
                Cell::from(job.name.clone()),
                Cell::from(format!("{}/{}", job.successful, completions)),
                Cell::from(duration),
                Cell::from(job.status.clone()).style(Style::default().fg(status_color)),
                Cell::from(job.age.clone()),
            ]).style(style)
        })
        .collect();

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Name", "Completions", "Duration", "Status", "Age"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Jobs ({})", app.jobs.len()))
        )
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(table, area, &mut create_table_state(app.selected_job_index));
}

fn create_table_state(selected: usize) -> ratatui::widgets::TableState {
    let mut state = ratatui::widgets::TableState::default();
    state.select(Some(selected));
    state
}