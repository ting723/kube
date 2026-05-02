# TUI UX Enhancement Design

**Date**: 2026-05-02
**Branch**: master
**Status**: approved

## Overview

Three lightweight UX enhancements for kube-tui, targeting daily inspection and development debugging workflows on minikube.

## Feature 1: Split-pane Log Viewer

### Purpose
Compare logs from two pods side-by-side without switching views.

### State Changes (`src/app/state.rs`)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ActivePane { Left, Right }

// Fields to add to AppState:
pub split_log_mode: bool,           // split-pane active flag
pub split_log_pod_name: String,     // right pane pod name
pub split_log_content: Vec<String>, // right pane log lines
pub split_log_scroll: usize,        // right pane scroll position
pub active_pane: ActivePane,        // currently focused pane
```

### Key Bindings

| Key | Context | Action |
|-----|---------|--------|
| `V` | Logs mode | Toggle split-pane; on enter, show pod picker |
| `Tab` | Split-pane active | Switch focus Left <-> Right |
| `j/k` | Split-pane active | Scroll focused pane |
| `PgUp/PgDn` | Split-pane active | Page-scroll focused pane |
| `Esc` | Split-pane active | Exit split-pane, keep focused pane |

### Rendering
- Split: `Layout::horizontal([50%, 50%])`
- Active pane: yellow border; inactive: gray border
- Each pane header: pod name + scroll position

## Feature 2: Batch Operations

### Purpose
Mark multiple list items and perform bulk actions.

### State Changes

```rust
pub marked_items: HashSet<usize>,   // indices of marked items
```

### Key Bindings

| Key | Context | Action |
|-----|---------|--------|
| `v` | Any list mode | Toggle batch mode |
| `Space` | Batch mode | Toggle mark on current item |
| `Ctrl+A` | Batch mode | Select all items |
| `d` | Batch mode | Delete marked items (confirm) |
| `Esc` | Batch mode | Exit, clear marks |

### Visual Feedback
- Marked items: `✓` prefix
- Footer: `[Batch Mode] Marked: N`

## Feature 3: Exec Flow Optimization

### Purpose
Return to TUI after `kubectl exec` without full data reload.

### Changes
- Skip `app.logs.clear()` and `app.describe_content.clear()` after exec
- Only incremental Pod list refresh
- Preserve scroll positions

## File Change Summary

| File | Lines | Change |
|------|-------|--------|
| `src/app/state.rs` | +20 | ActivePane enum, split_log fields, marked_items |
| `src/app/key_handler.rs` | +60 | V, Tab, v, Space, Ctrl+A, d handling |
| `src/ui/components/logs.rs` | +80 | Split-pane rendering |
| `src/ui/mod.rs` | +15 | Batch mode footer |
| `src/main.rs` | +40 | Split-pane data loading, exec optimization |
| **Total** | **~215** | |

## Test Plan

### Regression Script: `scripts/regression-test.sh`

```bash
#!/bin/bash
set -euo pipefail

echo "=== Kube TUI Regression Test ==="

# 1. Compile (0 warnings)
echo "[1/6] cargo check"
cargo check 2>&1 | grep -q "warning" && exit 1 || echo "  PASS"

# 2. Unit tests
echo "[2/6] cargo test"
cargo test 2>&1 | grep -q "test result: ok" && echo "  PASS" || exit 1

# 3. Lint
echo "[3/6] cargo clippy"
cargo clippy -- -D warnings && echo "  PASS" || exit 1

# 4. Format
echo "[4/6] cargo fmt --check"
cargo fmt -- --check && echo "  PASS" || exit 1

# 5. Minikube integration
echo "[5/6] minikube smoke test"
kubectl create namespace kube-tui-test --dry-run=client -o yaml | kubectl apply -f -
kubectl run test-nginx -n kube-tui-test --image=nginx:alpine --dry-run=client -o yaml | kubectl apply -f -
kubectl run test-busybox -n kube-tui-test --image=busybox -- sleep 3600 --dry-run=client -o yaml | kubectl apply -f -
kubectl wait --for=condition=Ready pods -n kube-tui-test --all --timeout=60s
kubectl get pods -n kube-tui-test | grep test-nginx || exit 1
kubectl get pods -n kube-tui-test | grep test-busybox || exit 1
echo "  PASS"

# 6. Cleanup
echo "[6/6] cleanup"
kubectl delete namespace kube-tui-test --ignore-not-found
echo "  PASS"

echo "=== All regression tests passed ==="
```

### Manual Test Cases

| # | Scenario | Steps | Expected |
|---|----------|-------|----------|
| 1 | List nav | j/k in PodList | Selection moves |
| 2 | Describe scroll | Space → j/k | Content scrolls |
| 3 | YAML scroll | Y → j/k | YAML scrolls |
| 4 | Single-pane logs | L → j/k | Log lines scroll |
| 5 | Split-pane logs | L → V → pick 2nd pod | Two panes |
| 6 | Split focus | Tab in split-pane | Focus switches |
| 7 | Split scroll | j/k per pane | Independent scroll |
| 8 | Exit split | V in split-pane | Single pane |
| 9 | Batch enter | v in PodList | Batch mode on |
| 10 | Batch mark | Space in batch mode | ✓ appears |
| 11 | Batch all | Ctrl+A in batch mode | All marked |
| 12 | Batch delete | d → confirm | Marked deleted |
| 13 | Batch exit | Esc in batch mode | Marks cleared |
| 14 | Exec flow | E → exit shell | Data preserved |
| 15 | Language | I | ZH <-> EN |
| 16 | Mouse mode | M in Describe/YAML | Mode toggles |

## Non-Goals
- Multi-cluster support
- Port-forwarding UI
- Event/watch streaming
- Custom keybinding configuration
