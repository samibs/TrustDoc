#!/bin/bash
# Security Check Script for TDF Format
# Run this script to perform automated security checks

set -e

echo "🔒 TDF Security Check"
echo "===================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Function to check command
check_cmd() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 not installed"
        echo "   Install with: $2"
        ((ERRORS++))
        return 1
    fi
}

# Function to run check
run_check() {
    echo ""
    echo "Running: $1"
    if eval "$2"; then
        echo -e "${GREEN}✓${NC} $1 passed"
    else
        echo -e "${RED}✗${NC} $1 failed"
        ((ERRORS++))
    fi
}

echo "1. Checking Security Tools"
echo "---------------------------"
check_cmd "cargo-audit" "cargo install cargo-audit"
check_cmd "cargo-deny" "cargo install cargo-deny --locked"
check_cmd "cargo-clippy" "rustup component add clippy"

echo ""
echo "2. Running Security Checks"
echo "--------------------------"

# Dependency audit
if check_cmd "cargo-audit" "cargo install cargo-audit" > /dev/null 2>&1; then
    run_check "Dependency Audit" "cargo audit"
else
    echo -e "${YELLOW}⚠${NC} Skipping dependency audit (cargo-audit not installed)"
    ((WARNINGS++))
fi

# Cargo deny
if check_cmd "cargo-deny" "cargo install cargo-deny" > /dev/null 2>&1; then
    run_check "License Check" "cargo deny check licenses" || true
    run_check "Banned Dependencies" "cargo deny check bans" || true
    run_check "Source Check" "cargo deny check sources" || true
else
    echo -e "${YELLOW}⚠${NC} Skipping cargo-deny checks (not installed)"
    ((WARNINGS++))
fi

# Clippy
run_check "Clippy (Lints)" "cargo clippy --all-targets -- -D warnings" || true

# Format check
run_check "Code Format" "cargo fmt --check" || true

# Tests
echo ""
echo "3. Running Security Tests"
echo "-------------------------"
run_check "Security Unit Tests" "cargo test --test security_tests"
run_check "E2E Security Tests" "cargo test --test e2e_security_tests"

# Summary
echo ""
echo "===================="
echo "Security Check Summary"
echo "===================="
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ All critical checks passed${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠ $WARNINGS warnings (optional tools not installed)${NC}"
    fi
    exit 0
else
    echo -e "${RED}✗ $ERRORS errors found${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠ $WARNINGS warnings${NC}"
    fi
    exit 1
fi

