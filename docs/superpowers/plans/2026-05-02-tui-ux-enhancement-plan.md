# TUI UX Enhancement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add split-pane log viewer, batch operations, and exec flow optimization to kube-tui (~215 net new lines across 5 files).

**Architecture:** Extend AppState with split-pane and batch state fields; add key handler branches for V/Tab/v/Space/Ctrl+A/d; modify logs.rs renderer for horizontal split layout; optimize main.rs exec path to preserve existing data.

**Tech Stack:** Rust 2024 edition, ratatui 0.30, crossterm 0.29, tokio

---

## File Structure

| File | Role |
|------|------|
| `src/app/state.rs` | Add `ActivePane` enum, split-log fields, `marked_items`, `exec_returning`; update scroll methods |
| `src/app/key_handler.rs` | Add V/Tab (split-pane), v/Space/Ctrl+A/d (batch), update j/k/PgUp/PgDn for active pane |
| `src/ui/components/logs.rs` | Split-pane rendering: dual Paragraph with independent scroll state |
| `src/ui/mod.rs` | Update footer hints for batch mode |
| `src/main.rs` | Load right-pane logs, optimize post-exec reload |

---

### Task 1: Add state fields and methods

**Files:**
- Modify: `src/app/state.rs`

- [ ] **Step 1: Add HashSet back to imports and add ActivePane enum**

Line 1, change `use std::collections::HashMap;` to `use std::collections::{HashMap, HashSet};`

After the `AppMode` enum (after line 38), add:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ActivePane { Left, Right }
```

- [ ] **Step 2: Add new fields to AppState struct**

After `pub batch_mode: bool,`, add:
```rust
pub split_log_mode: bool,
pub split_log_pod_name: String,
pub split_log_content: Vec<String>,
pub split_log_scroll: usize,
pub active_pane: ActivePane,
pub marked_items: HashSet<usize>,
pub exec_returning: bool,
```

- [ ] **Step 3: Add fields to Default impl**

In `AppState::default()`, after `batch_mode: false,`, add:
```rust
split_log_mode: false,
split_log_pod_name: String::new(),
split_log_content: Vec::new(),
split_log_scroll: 0,
active_pane: ActivePane::Left,
marked_items: HashSet::new(),
exec_returning: false,
```

- [ ] **Step 4: Add new methods to impl AppState**

Before the `#[cfg(test)]` block, add:
```rust
pub fn toggle_batch_mode(&mut self) {
    self.batch_mode = !self.batch_mode;
    if !self.batch_mode {
        self.marked_items.clear();
    }
}

pub fn toggle_mark_current(&mut self) {
    if !self.batch_mode { return; }
    let idx = self.current_selection_index();
    if self.marked_items.contains(&idx) {
        self.marked_items.remove(&idx);
    } else {
        self.marked_items.insert(idx);
    }
}

pub fn mark_all(&mut self) {
    if !self.batch_mode { return; }
    let count = self.current_list_len();
    for i in 0..count {
        self.marked_items.insert(i);
    }
}

fn current_selection_index(&self) -> usize {
    match self.mode {
        AppMode::NamespaceList => self.selected_namespace_index,
        AppMode::PodList => self.selected_pod_index,
        AppMode::ServiceList => self.selected_service_index,
        AppMode::NodeList => self.selected_node_index,
        AppMode::DeploymentList => self.selected_deployment_index,
        AppMode::JobList => self.selected_job_index,
        AppMode::DaemonSetList => self.selected_daemonset_index,
        AppMode::PVCList => self.selected_pvc_index,
        AppMode::PVList => self.selected_pv_index,
        AppMode::ConfigMapList => self.selected_configmap_index,
        AppMode::SecretList => self.selected_secret_index,
        _ => 0,
    }
}

fn current_list_len(&self) -> usize {
    match self.mode {
        AppMode::NamespaceList => self.namespaces.len(),
        AppMode::PodList => self.pods.len(),
        AppMode::ServiceList => self.services.len(),
        AppMode::NodeList => self.nodes.len(),
        AppMode::DeploymentList => self.deployments.len(),
        AppMode::JobList => self.jobs.len(),
        AppMode::DaemonSetList => self.daemonsets.len(),
        AppMode::PVCList => self.pvcs.len(),
        AppMode::PVList => self.pvs.len(),
        AppMode::ConfigMapList => self.configmaps.len(),
        AppMode::SecretList => self.secrets.len(),
        _ => 0,
    }
}
```

- [ ] **Step 5: Update scroll methods for split-pane support**

In `scroll_up` (line 406), replace `AppMode::Logs => { if self.logs_scroll > 0 { self.logs_scroll -= 1; } }` with:
```rust
AppMode::Logs => {
    if self.split_log_mode {
        match self.active_pane {
            ActivePane::Left => { if self.logs_scroll > 0 { self.logs_scroll -= 1; } }
            ActivePane::Right => { if self.split_log_scroll > 0 { self.split_log_scroll -= 1; } }
        }
    } else {
        if self.logs_scroll > 0 { self.logs_scroll -= 1; }
    }
}
```

In `scroll_down` (line 432), replace `AppMode::Logs => { ... }` with:
```rust
AppMode::Logs => {
    if self.split_log_mode {
        match self.active_pane {
            ActivePane::Left => { if self.logs_scroll + 1 < self.logs.len() { self.logs_scroll += 1; } }
            ActivePane::Right => { if self.split_log_scroll + 1 < self.split_log_content.len() { self.split_log_scroll += 1; } }
        }
    } else {
        if self.logs_scroll + 1 < self.logs.len() { self.logs_scroll += 1; }
    }
}
```

In `scroll_page_up` (line 458), replace `AppMode::Logs => { self.logs_scroll = self.logs_scroll.saturating_sub(10); }` with:
```rust
AppMode::Logs => {
    if self.split_log_mode {
        match self.active_pane {
            ActivePane::Left => self.logs_scroll = self.logs_scroll.saturating_sub(10),
            ActivePane::Right => self.split_log_scroll = self.split_log_scroll.saturating_sub(10),
        }
    } else {
        self.logs_scroll = self.logs_scroll.saturating_sub(10);
    }
}
```

In `scroll_page_down` (line 476), replace `AppMode::Logs => { ... }` with:
```rust
AppMode::Logs => {
    if self.split_log_mode {
        match self.active_pane {
            ActivePane::Left => {
                let max = self.logs.len().saturating_sub(1);
                self.logs_scroll = (self.logs_scroll + 10).min(max);
            }
            ActivePane::Right => {
                let max = self.split_log_content.len().saturating_sub(1);
                self.split_log_scroll = (self.split_log_scroll + 10).min(max);
            }
        }
    } else {
        let max_scroll = self.logs.len().saturating_sub(1);
        self.logs_scroll = (self.logs_scroll + 10).min(max_scroll);
    }
}
```

In `reset_scroll`, after `self.logs_scroll = 0;`, add: `self.split_log_scroll = 0;`

- [ ] **Step 6: Add tests**

Add to the `#[cfg(test)] mod tests` block:
```rust
#[test]
fn test_batch_mode_toggle() {
    let mut state = AppState::default();
    state.mode = AppMode::PodList;
    state.pods.push(crate::kubectl::types::Pod {
        name: "test-pod".into(), namespace: "default".into(),
        status: crate::kubectl::types::PodStatus { phase: "Running".into(), conditions: None, container_statuses: None },
        ready: "1/1".into(), restarts: 0, age: "1d".into(), node: None, ip: None,
    });
    state.toggle_batch_mode();
    assert!(state.batch_mode);
    state.toggle_mark_current();
    assert_eq!(state.marked_items.len(), 1);
    state.toggle_batch_mode();
    assert!(!state.batch_mode);
    assert!(state.marked_items.is_empty());
}

#[test]
fn test_mark_all() {
    let mut state = AppState::default();
    state.mode = AppMode::PodList;
    for i in 0..3 {
        state.pods.push(crate::kubectl::types::Pod {
            name: format!("pod-{}", i), namespace: "default".into(),
            status: crate::kubectl::types::PodStatus { phase: "Running".into(), conditions: None, container_statuses: None },
            ready: "1/1".into(), restarts: 0, age: "1d".into(), node: None, ip: None,
        });
    }
    state.toggle_batch_mode();
    state.mark_all();
    assert_eq!(state.marked_items.len(), 3);
}

#[test]
fn test_split_log_mode_defaults() {
    let state = AppState::default();
    assert!(!state.split_log_mode);
    assert_eq!(state.split_log_scroll, 0);
}

#[test]
fn test_active_pane_scroll() {
    let mut state = AppState::default();
    state.mode = AppMode::Logs;
    state.split_log_mode = true;
    state.split_log_content = vec!["a".into(), "b".into(), "c".into()];
    state.active_pane = ActivePane::Right;
    state.scroll_down();
    assert_eq!(state.split_log_scroll, 1);
    state.scroll_up();
    assert_eq!(state.split_log_scroll, 0);
    state.active_pane = ActivePane::Left;
    state.logs = vec!["x".into(), "y".into()];
    state.logs_scroll = 0;
    state.scroll_down();
    assert_eq!(state.logs_scroll, 1);
    assert_eq!(state.split_log_scroll, 0);
}
```

- [ ] **Step 7: Compile check and run tests**

Run: `cargo check 2>&1 | grep -c warning` → Expect: `0`
Run: `cargo test` → Expect: all pass

- [ ] **Step 8: Commit**

```bash
git add src/app/state.rs
git commit -m "feat: add split-pane log viewer and batch operations state"
```

---

### Task 2: Add key handler logic for split-pane and batch mode

**Files:**
- Modify: `src/app/key_handler.rs`

- [ ] **Step 1: Update imports**

Line 2, change to: `use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};`
Line 3, change to: `use super::state::{AppState, ActivePane, ConfirmAction, AppMode};`

- [ ] **Step 2: Update Esc handler for batch/split exit**

Before the `match self.mode {` in the Esc handler, add:
```rust
KeyCode::Esc => {
    if self.batch_mode {
        self.toggle_batch_mode();
        return Ok(());
    }
    if self.split_log_mode {
        self.split_log_mode = false;
    }
    match self.mode {
        // ... existing match arms ...
    }
}
```

- [ ] **Step 3: Add V key for split-pane toggle**

Insert before `KeyCode::Char('j')`:
```rust
KeyCode::Char('V') => {
    if self.mode == AppMode::Logs {
        if self.split_log_mode {
            self.split_log_mode = false;
        } else {
            self.enter_split_log_mode();
        }
    }
}
```

- [ ] **Step 4: Update Tab for split-pane focus switch**

Change `KeyCode::Tab => self.switch_panel(),` to:
```rust
KeyCode::Tab => {
    if self.mode == AppMode::Logs && self.split_log_mode {
        self.active_pane = match self.active_pane {
            ActivePane::Left => ActivePane::Right,
            ActivePane::Right => ActivePane::Left,
        };
    } else {
        self.switch_panel();
    }
}
```

- [ ] **Step 5: Add v key for batch mode toggle**

Add after Tab handling:
```rust
KeyCode::Char('v') => {
    match self.mode {
        AppMode::NamespaceList | AppMode::PodList | AppMode::ServiceList
        | AppMode::NodeList | AppMode::DeploymentList | AppMode::JobList
        | AppMode::DaemonSetList | AppMode::PVCList | AppMode::PVList
        | AppMode::ConfigMapList | AppMode::SecretList => {
            self.toggle_batch_mode();
        }
        _ => {}
    }
}
```

- [ ] **Step 6: Update Space for batch mark**

Change `KeyCode::Char(' ') => self.handle_describe(),` to:
```rust
KeyCode::Char(' ') => {
    if self.batch_mode {
        self.toggle_mark_current();
    } else {
        self.handle_describe();
    }
}
```

- [ ] **Step 7: Add Ctrl+A select-all and d batch-delete**

Add these cases:
```rust
KeyCode::Char('a') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
    if self.batch_mode {
        self.mark_all();
    }
}
KeyCode::Char('d') => {
    if self.batch_mode && !self.marked_items.is_empty() {
        let items: Vec<(String, String, String)> = self.marked_items.iter()
            .filter_map(|&i| self.pods.get(i))
            .map(|p| (self.current_namespace.clone(), "pod".to_string(), p.name.clone()))
            .collect();
        self.confirm_action = Some(ConfirmAction::DeleteBatch { items });
        self.mode = AppMode::Confirm;
    }
}
```

- [ ] **Step 8: Add enter_split_log_mode method**

Add to the impl block (before tests):
```rust
pub fn enter_split_log_mode(&mut self) {
    self.split_log_mode = true;
    self.split_log_scroll = 0;
    self.active_pane = ActivePane::Right;
    let next_idx = if self.selected_pod_index + 1 < self.pods.len() {
        self.selected_pod_index + 1
    } else {
        self.selected_pod_index.saturating_sub(1)
    };
    if let Some(pod) = self.pods.get(next_idx) {
        self.split_log_pod_name = pod.name.clone();
    }
}
```

- [ ] **Step 9: Add tests**

```rust
#[test]
fn test_batch_mode_key() {
    let mut state = create_test_state();
    state.mode = AppMode::PodList;
    let key = KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE);
    state.handle_key_event(key).unwrap();
    assert!(state.batch_mode);
    state.handle_key_event(key).unwrap();
    assert!(!state.batch_mode);
}

#[test]
fn test_split_log_v_key() {
    let mut state = create_test_state();
    state.mode = AppMode::Logs;
    state.pods.push(crate::kubectl::types::Pod {
        name: "pod1".into(), namespace: "default".into(),
        status: crate::kubectl::types::PodStatus { phase: "Running".into(), conditions: None, container_statuses: None },
        ready: "1/1".into(), restarts: 0, age: "1d".into(), node: None, ip: None,
    });
    state.pods.push(crate::kubectl::types::Pod {
        name: "pod2".into(), namespace: "default".into(),
        status: crate::kubectl::types::PodStatus { phase: "Running".into(), conditions: None, container_statuses: None },
        ready: "1/1".into(), restarts: 0, age: "1d".into(), node: None, ip: None,
    });
    let v_key = KeyEvent::new(KeyCode::Char('V'), KeyModifiers::NONE);
    state.handle_key_event(v_key).unwrap();
    assert!(state.split_log_mode);
    assert_eq!(state.split_log_pod_name, "pod2");
}
```

- [ ] **Step 10: Compile check and run tests**

Run: `cargo check 2>&1 | grep -c warning` → Expect: `0`
Run: `cargo test` → Expect: all pass

- [ ] **Step 11: Commit**

```bash
git add src/app/key_handler.rs
git commit -m "feat: add split-pane and batch mode key handling"
```

---

### Task 3: Add split-pane log rendering

**Files:**
- Modify: `src/ui/components/logs.rs`

- [ ] **Step 1: Replace render function with split-pane version**

```rust
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};
use crate::app::state::{ActivePane, AppState};

fn log_pane_title(app: &AppState, pod_name: &str, pane: &ActivePane) -> String {
    let marker = match pane {
        ActivePane::Left if app.active_pane == ActivePane::Left => "◉",
        ActivePane::Right if app.active_pane == ActivePane::Right => "◉",
        _ => " ",
    };
    if app.language_chinese {
        format!("{} 日志 - {}/{}", marker, app.current_namespace, pod_name)
    } else {
        format!("{} Logs - {}/{}", marker, app.current_namespace, pod_name)
    }
}

fn render_log_pane(f: &mut Frame, area: Rect, logs: &[String], scroll: usize, title: &str) {
    if logs.is_empty() {
        let widget = Paragraph::new("Loading logs...")
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(widget, area);
        return;
    }
    let content = logs.iter().enumerate()
        .map(|(i, line)| format!("[{}] {}", i + 1, line))
        .collect::<Vec<_>>().join("\n");
    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .scroll((scroll as u16, 0));
    f.render_widget(paragraph, area);
    let total = logs.len();
    let visible = area.height.saturating_sub(2) as usize;
    if total > visible {
        let mut state = ScrollbarState::default()
            .content_length(total).viewport_content_length(visible).position(scroll);
        let sb = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑")).end_symbol(Some("↓"));
        f.render_stateful_widget(sb, area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }), &mut state);
    }
}

pub fn render(f: &mut Frame, area: Rect, app: &AppState) {
    if app.split_log_mode {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let left_name = app.pods.get(app.selected_pod_index).map(|p| p.name.as_str()).unwrap_or("?");
        render_log_pane(f, panes[0], &app.logs, app.logs_scroll, &log_pane_title(app, left_name, &ActivePane::Left));
        render_log_pane(f, panes[1], &app.split_log_content, app.split_log_scroll, &log_pane_title(app, &app.split_log_pod_name, &ActivePane::Right));
    } else {
        let name = app.pods.get(app.selected_pod_index).map(|p| p.name.as_str()).unwrap_or("?");
        let title = if app.language_chinese {
            format!("日志 - {}/{}", app.current_namespace, name)
        } else {
            format!("Logs - {}/{}", app.current_namespace, name)
        };
        render_log_pane(f, area, &app.logs, app.logs_scroll, &title);
    }
}
```

- [ ] **Step 2: Compile check**

Run: `cargo check` → Expect: 0 warnings

- [ ] **Step 3: Commit**

```bash
git add src/ui/components/logs.rs
git commit -m "feat: add split-pane log viewer rendering"
```

---

### Task 4: Update footer hints for batch mode

**Files:**
- Modify: `src/ui/mod.rs`

- [ ] **Step 1: Change help_text to String and add batch/split hints**

Change `let help_text = if app.language_chinese {` to `let help_text: String = if app.language_chinese {`

For every list mode arm, add a `batch_mode` check at the top. Example for PodList:
```rust
AppMode::PodList => {
    if app.batch_mode {
        if app.language_chinese {
            format!("v 退出批量 | Space 标记 | Ctrl+A 全选 | d 删除 | Esc 取消 | 已标记: {}", app.marked_items.len())
        } else {
            format!("v Exit Batch | Space Mark | Ctrl+A All | d Delete | Esc Cancel | Marked: {}", app.marked_items.len())
        }
    } else {
        if app.language_chinese {
            "j/k 导航 • Space 详情 • Y YAML • T 监控 • L 日志 • D 删除 • E 进入 • v 批量 • / 搜索 • I 语言 • q 退出 • R 刷新".to_string()
        } else {
            "j/k Navigate • Space Describe • Y YAML • T Top • L Logs • D Delete • E Exec • v Batch • / Search • I Lang • q Quit • R Refresh".to_string()
        }
    }
}
```

Apply the same pattern (batch_mode check + existing text with `.to_string()`) to ALL list mode arms.

Update Logs hint to add V:
```rust
AppMode::Logs => {
    if app.language_chinese {
        "j/k 滚动 • PgUp/PgDn 翻页 • V 分屏日志 • A 自动滚动 • R 自动刷新 • M 鼠标 • I 语言 • Esc 返回 • q 退出".to_string()
    } else {
        "j/k Scroll • PgUp/PgDn Page • V Split Logs • A Auto-scroll • R Refresh • M Mouse • I Lang • Esc Back • q Quit".to_string()
    }
}
```

- [ ] **Step 2: Compile check**

Run: `cargo check` → Expect: 0 warnings

- [ ] **Step 3: Commit**

```bash
git add src/ui/mod.rs
git commit -m "feat: add batch mode and split-log footer hints"
```

---

### Task 5: Update main.rs for split-pane loading and exec optimization

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Optimize exec return flow (line ~150-182)**

Replace:
```rust
// 在exec后强制刷新界面和数据
app.logs.clear();
app.describe_content.clear();
// 强制返回到PodList模式并重新加载所有数据
app.mode = AppMode::PodList;
app.previous_mode = AppMode::PodList;
// 重新加载所有必要的数据
if let Ok(pods) = client.get_pods(&app.current_namespace).await { app.pods = pods; }
if let Ok(services) = client.get_services(&app.current_namespace).await { app.services = services; }
if let Ok(deployments) = client.get_deployments(&app.current_namespace).await { app.deployments = deployments; }
if let Ok(jobs) = client.get_jobs(&app.current_namespace).await { app.jobs = jobs; }
app.refresh_data();
terminal.draw(|f| ui::render_ui(f, app))?;
```

With:
```rust
app.mode = AppMode::PodList;
app.previous_mode = AppMode::PodList;
if let Ok(pods) = client.get_pods(&app.current_namespace).await { app.pods = pods; }
app.refresh_data();
terminal.draw(|f| ui::render_ui(f, app))?;
```

- [ ] **Step 2: Add split-pane log loading in Logs initial load (around line ~300, after main log load)**

```rust
if app.split_log_mode && !app.split_log_pod_name.is_empty() {
    let split_ns = app.current_namespace.clone();
    let split_name = app.split_log_pod_name.clone();
    if let Ok(split_logs) = client.get_pod_logs(&split_ns, &split_name, 100).await {
        app.split_log_content = split_logs;
    }
}
```

- [ ] **Step 3: Add split-pane log loading in auto-refresh Logs block (around line ~670)**

Add the same split log loading snippet after the main log load in the auto-refresh section.

- [ ] **Step 4: Compile check**

Run: `cargo check` → Expect: 0 warnings

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: add split-pane log data loading and exec flow optimization"
```

---

### Task 6: Final verification

- [ ] **Step 1: Run regression test**

```bash
bash scripts/regression-test.sh
```
Expected: all 6 steps PASS, 0 warnings.

- [ ] **Step 2: Run all unit tests**

```bash
cargo test -- --nocapture
```
Expected: all tests pass.

- [ ] **Step 3: Verify with minikube**

```bash
cargo build --release
timeout 3 ./target/release/kube-tui 2>&1 || true
```

- [ ] **Step 4: Commit any final fixes**

```bash
git add -A && git commit -m "chore: final verification and cleanup"
```
