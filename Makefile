# Makefile for Rust project

# Variables
CARGO := cargo
TARGET_DIR := target
BINARY_NAME := orka
PATH_ARG ?= .

# Colors
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

# Default target
.PHONY: all
all: build



# Build the project
.PHONY: build
build:
	@echo "$(YELLOW)Building $(BINARY_NAME)...$(NC)"
	$(CARGO) build --release
	@echo "\n$(GREEN)Build complete: $(TARGET_DIR)/release/$(BINARY_NAME)$(NC)"

# Run tests
.PHONY: test
test:
	@echo "$(YELLOW)Running tests for codebase...$(NC)"
	$(CARGO) test --verbose
	@echo "$(GREEN)All tests passed!$(NC)"

# Run CI/CD checks (fmt, clippy, tests)
.PHONY: check
check: fmt clippy test
	@echo "$(GREEN)All CI/CD checks passed!$(NC)"

# Rustfmt check
.PHONY: fmt
fmt:
	@echo "$(YELLOW)Checking code formatting with rustfmt...$(NC)"
	$(CARGO) fmt -- --check
	@echo "$(GREEN)Formatting check passed!$(NC)"

# Clippy check
.PHONY: clippy
clippy:
	@echo "$(YELLOW)Running Clippy linter...$(NC)"
	$(CARGO) clippy -- -D warnings
	@echo "$(GREEN)Clippy check passed!$(NC)"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	@echo "$(GREEN)Clean complete!$(NC)"

# Run audit (requires cargo-audit)
.PHONY: audit
audit:
	@echo "$(YELLOW)Running security audit...$(NC)"
	cargo audit
	@echo "$(GREEN)Audit complete!$(NC)"

# Format code automatically
.PHONY: format
format:
	@echo "$(YELLOW)Auto-formatting code...$(NC)"
	$(CARGO) fmt
	@echo "$(GREEN)Formatting complete!$(NC)"

# Help target
.PHONY: help
help:
	@echo "$(GREEN)Available targets:$(NC)"
	@echo "  $(YELLOW)make build$(NC)            - Build the project in release mode"
	@echo "  $(YELLOW)make test$(NC)             - Run all tests"
	@echo "  $(YELLOW)make check$(NC)            - Run fmt, clippy, and test (CI/CD checks)"
	@echo "  $(YELLOW)make fmt$(NC)              - Check code formatting"
	@echo "  $(YELLOW)make clippy$(NC)           - Run Clippy linter"
	@echo "  $(YELLOW)make clean$(NC)            - Clean build artifacts"
	@echo "  $(YELLOW)make format$(NC)           - Auto-format code"
	@echo "  $(YELLOW)make audit$(NC)            - Run security audit"
	@echo "  $(YELLOW)make help$(NC)             - Show this help message"
