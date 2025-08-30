# Makefile for Kube TUI

.PHONY: build build-release clean install test lint format

# 默认目标
build:
	cargo build

# 构建发布版本
build-release:
	cargo build --release

# 清理构建产物
clean:
	cargo clean

# 安装到系统
install: build-release
	@echo "Installing kube-tui..."
	@if [ "$(shell uname)" = "Darwin" ] || [ "$(shell uname)" = "Linux" ]; then \
		sudo cp target/release/kube-tui /usr/local/bin/; \
		echo "✅ Installed to /usr/local/bin/kube-tui"; \
	else \
		echo "Please manually copy target/release/kube-tui to your PATH"; \
	fi

# 运行测试
test:
	cargo test

# 代码检查
lint:
	cargo clippy -- -D warnings

# 代码格式化
format:
	cargo fmt

# 检查格式化
format-check:
	cargo fmt -- --check

# 构建所有平台的二进制文件
build-all:
	@echo "Building for multiple platforms..."
	@# Linux x86_64
	@if command -v cargo > /dev/null 2>&1; then \
		echo "Building for Linux x86_64..."; \
		cargo build --release --target x86_64-unknown-linux-gnu; \
	fi
	@# macOS x86_64
	@if command -v cargo > /dev/null 2>&1; then \
		echo "Building for macOS x86_64..."; \
		cargo build --release --target x86_64-apple-darwin; \
	fi
	@# macOS ARM64
	@if command -v cargo > /dev/null 2>&1; then \
		echo "Building for macOS ARM64..."; \
		cargo build --release --target aarch64-apple-darwin; \
	fi

# 打包发布
package: build-release
	@echo "Creating release package..."
	@mkdir -p dist
	@cp target/release/kube-tui dist/
	@cp README.md dist/
	@cd dist && tar -czf kube-tui-$(shell uname -m)-$(shell uname -s).tar.gz kube-tui README.md
	@echo "Package created: dist/kube-tui-$(shell uname -m)-$(shell uname -s).tar.gz"

# 开发模式运行
dev:
	cargo run

# 发布前检查
pre-release: format-check lint test build-release
	@echo "✅ All checks passed!"

help:
	@echo "Available targets:"
	@echo "  build         - Build debug version"
	@echo "  build-release - Build release version"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install to system PATH"
	@echo "  test          - Run tests"
	@echo "  lint          - Run linter"
	@echo "  format        - Format code"
	@echo "  format-check  - Check code formatting"
	@echo "  build-all     - Build for multiple platforms"
	@echo "  package       - Create release package"
	@echo "  dev           - Run in development mode"
	@echo "  pre-release   - Run all pre-release checks"
	@echo "  help          - Show this help"