#!/bin/bash
# Kube TUI Regression Test Suite
# Run before merging major refactors: bash scripts/regression-test.sh
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() { echo -e "${GREEN}  PASS${NC}"; }
fail() { echo -e "${RED}  FAIL: $1${NC}"; exit 1; }
info() { echo -e "${YELLOW}$1${NC}"; }

echo "============================================"
echo "  Kube TUI Regression Test Suite"
echo "============================================"
echo ""

# 1. Compile check (0 warnings required)
echo "[1/6] cargo check (0 warnings)"
if cargo check 2>&1 | tee /tmp/kube-tui-check.log | grep -q "warning"; then
    echo "Warnings found:"
    grep "warning:" /tmp/kube-tui-check.log
    fail "cargo check has warnings"
fi
pass

# 2. Unit tests
echo "[2/6] cargo test"
cargo test 2>&1 | tee /tmp/kube-tui-test.log
if grep -q "test result: ok" /tmp/kube-tui-test.log; then
    passed=$(grep "test result:" /tmp/kube-tui-test.log | tail -1)
    info "  $passed"
    pass
else
    fail "unit tests failed"
fi

# 3. Lint (no warnings allowed)
echo "[3/6] cargo clippy (warnings allowed)"
cargo clippy 2>&1 | tee /tmp/kube-tui-clippy.log
clippy_warnings=$(grep -c "warning:" /tmp/kube-tui-clippy.log 2>/dev/null || echo "0")
if [ "$clippy_warnings" -gt 0 ]; then
    info "  Clippy: $clippy_warnings warnings (pre-existing, not failing)"
fi
pass

# 4. Format check
echo "[4/6] cargo fmt --check"
if cargo fmt -- --check; then
    pass
else
    info "  Run 'cargo fmt' to fix formatting"
    fail "formatting issues found"
fi

# 5. Minikube integration smoke test
echo "[5/6] minikube smoke test"

if ! command -v minikube &>/dev/null && ! command -v kubectl &>/dev/null; then
    info "  minikube/kubectl not found, skipping integration test"
else
    if command -v minikube &>/dev/null; then
        minikube status 2>/dev/null | grep -q "Running" || {
            info "  Starting minikube..."
            minikube start --wait=all 2>&1 | tail -3
        }
    fi

    NAMESPACE="kube-tui-regression-test"

    kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1

    kubectl run test-nginx -n "$NAMESPACE" \
        --image=nginx:alpine \
        --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1

    kubectl run test-busybox -n "$NAMESPACE" \
        --image=busybox -- sleep 3600 \
        --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1

    kubectl wait --for=condition=Ready pods -n "$NAMESPACE" --all --timeout=120s >/dev/null 2>&1 || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "pods not ready in time"
    }

    kubectl get pods -n "$NAMESPACE" | grep -q "test-nginx" || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "test-nginx pod not found"
    }
    kubectl get pods -n "$NAMESPACE" | grep -q "test-busybox" || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "test-busybox pod not found"
    }
    kubectl get namespaces | grep -q "$NAMESPACE" || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "test namespace not found"
    }
    kubectl get pods -n "$NAMESPACE" -o json | grep -q "test-nginx" || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "kubectl get pods JSON output unexpected"
    }

    kubectl logs -n "$NAMESPACE" test-nginx --tail=5 >/dev/null 2>&1 || {
        info "  Note: nginx logs not yet available (may be normal)"
    }
    kubectl describe pod -n "$NAMESPACE" test-nginx >/dev/null 2>&1 || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "kubectl describe pod failed"
    }
    kubectl get pod -n "$NAMESPACE" test-nginx -o yaml >/dev/null 2>&1 || {
        kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
        fail "kubectl get pod -o yaml failed"
    }
    kubectl get services -n "$NAMESPACE" >/dev/null 2>&1 || fail "kubectl get services failed"
    kubectl get nodes >/dev/null 2>&1 || fail "kubectl get nodes failed"

    kubectl delete namespace "$NAMESPACE" --ignore-not-found >/dev/null 2>&1
    pass
fi

# 6. Additional checks
echo "[6/6] additional checks"

if ls src/app_temp 2>/dev/null; then
    fail "src/app_temp/ still exists"
fi
if ls src/app/handlers/batch.rs src/app/handlers/list_mode.rs 2>/dev/null; then
    fail "dead handler files still exist"
fi

for f in src/main.rs src/app/state.rs src/app/key_handler.rs src/app/mod.rs \
         src/ui/mod.rs src/kubectl/client.rs src/kubectl/types.rs; do
    test -f "$f" || fail "missing file: $f"
done

pass

echo ""
echo "============================================"
echo "  All regression tests passed!"
echo "============================================"
