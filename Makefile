# Makefile for rustytime - Rust CLI time tracking application

.PHONY: help build test clean install dev fmt clippy check run migrate docs release

# Default target
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development commands
dev: check ## Run development checks (format, clippy, test)

check: fmt clippy test ## Run all checks

fmt: ## Format code with rustfmt
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

test: ## Run tests
	cargo test

# Build commands
build: ## Build the project in debug mode
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

# Installation commands
install: ## Install the binary from current directory
	cargo install --path .

install-force: ## Force reinstall the binary
	cargo install --path . --force

uninstall: ## Uninstall the binary
	cargo uninstall rustytime

# Database commands
init-db: ## Initialize database for local development
	cargo sqlx database create
	cargo sqlx migrate run

migrate: ## Run database migrations
	cargo run -- --help || echo "Database will be created on first run"

# Utility commands
clean: ## Clean build artifacts
	cargo clean

run: ## Run the application (shows help)
	cargo run

run-dev: ## Run the application in development mode with RUST_LOG=debug
	RUST_LOG=debug cargo run

# Documentation
docs: ## Generate and open documentation
	cargo doc --open

# Release commands
check-release: ## Check if ready for release
	cargo check --release
	cargo test --release
	cargo clippy --release -- -D warnings

release: check-release ## Build release binary
	cargo build --release
	@echo "Release binary created at target/release/rustytime"

# Project management examples (using the built binary)
example-usage: ## Show example usage commands
	@echo "Example usage after installation:"
	@echo "  rustytime project add \"My Project\" --desc \"Project description\""
	@echo "  rustytime project list"
	@echo "  rustytime task add \"My Task\" --project-id PROJECT_ID"
	@echo "  rustytime start TASK_ID"
	@echo "  rustytime stop"
	@echo "  rustytime report daily"
	@echo "  rustytime export json output.json"

# Development helpers
watch: ## Watch for changes and run tests
	@which cargo-watch > /dev/null || (echo "Installing cargo-watch..." && cargo install cargo-watch)
	cargo watch -x test

watch-run: ## Watch for changes and run the application
	@which cargo-watch > /dev/null || (echo "Installing cargo-watch..." && cargo install cargo-watch)
	cargo watch -x run

# Security and audit
audit: ## Run security audit
	@which cargo-audit > /dev/null || (echo "Installing cargo-audit..." && cargo install cargo-audit)
	cargo audit

# Update dependencies
update: ## Update dependencies
	cargo update

# All-in-one commands
setup: ## Initial setup for development
	rustup component add rustfmt clippy
	@echo "Development environment ready!"

ci: fmt clippy test build ## Run CI pipeline locally

deploy: clean check-release install ## Clean, build, test and install
	@echo "Deployment complete! rustytime is now installed."
