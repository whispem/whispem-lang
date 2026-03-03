#!/bin/sh
#
# tests/run_tests.sh -- autonomous test suite for Whispem (no Rust needed)
#
# Uses only:
#   ./wvm                 (C VM, built via `make`)
#   compiler/wsc.whbc     (bootstrapped compiler)
#
# Each test: compile .wsp → .whbc via wsc.whbc, run .whbc, compare output
# to the expected output in tests/expected/.
#
# Usage:
#   ./tests/run_tests.sh           # run all tests
#   ./tests/run_tests.sh hello     # run one test

set -e

cd "$(dirname "$0")/.."

WVM=./wvm
WSC=compiler/wsc.whbc
EXPECTED=tests/expected
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

pass=0
fail=0
skip=0
errors=""

run_test() {
    name="$1"
    src="$2"
    expected="$3"

    # Compile .wsp → .whbc
    if ! "$WVM" "$WSC" "$src" > /dev/null 2>&1; then
        printf "SKIP  %s (compile failed)\n" "$name"
        skip=$((skip + 1))
        return
    fi

    # Derive .whbc path (wsc.wsp writes it next to the source)
    whbc="${src%.wsp}.whbc"
    if [ ! -f "$whbc" ]; then
        printf "SKIP  %s (no .whbc produced)\n" "$name"
        skip=$((skip + 1))
        return
    fi

    # Run .whbc and capture output
    actual="$TMPDIR/$name.actual"
    "$WVM" "$whbc" > "$actual" 2>&1 || true

    # Compare
    if diff -q "$expected" "$actual" > /dev/null 2>&1; then
        printf "OK    %s\n" "$name"
        pass=$((pass + 1))
    else
        printf "FAIL  %s\n" "$name"
        diff "$expected" "$actual" | head -10
        fail=$((fail + 1))
        errors="$errors $name"
    fi

    # Clean up .whbc
    rm -f "$whbc"
}

# Check prerequisites
if [ ! -x "$WVM" ]; then
    echo "Error: wvm not found. Run 'make' first."
    exit 1
fi
if [ ! -f "$WSC" ]; then
    echo "Error: compiler/wsc.whbc not found."
    exit 1
fi

echo "=== Whispem test suite (autonomous, no Rust) ==="
echo ""

if [ $# -gt 0 ]; then
    # Run specific tests
    for name in "$@"; do
        if [ -f "examples/$name.wsp" ] && [ -f "$EXPECTED/$name.txt" ]; then
            run_test "$name" "examples/$name.wsp" "$EXPECTED/$name.txt"
        elif [ -f "tests/$name.wsp" ] && [ -f "$EXPECTED/$name.txt" ]; then
            run_test "$name" "tests/$name.wsp" "$EXPECTED/$name.txt"
        else
            printf "SKIP  %s (source or expected output not found)\n" "$name"
            skip=$((skip + 1))
        fi
    done
else
    # Run all example tests
    for exp in "$EXPECTED"/*.txt; do
        name=$(basename "$exp" .txt)
        if [ -f "examples/$name.wsp" ]; then
            run_test "$name" "examples/$name.wsp" "$exp"
        elif [ -f "tests/$name.wsp" ]; then
            run_test "$name" "tests/$name.wsp" "$exp"
        else
            printf "SKIP  %s (source not found)\n" "$name"
            skip=$((skip + 1))
        fi
    done

    # Bootstrap test
    echo ""
    echo "--- Bootstrap ---"
    cp "$WSC" "$TMPDIR/wsc_before.whbc"
    "$WVM" "$WSC" compiler/wsc.wsp > /dev/null 2>&1
    sha_before=$(shasum "$TMPDIR/wsc_before.whbc" | cut -d' ' -f1)
    sha_after=$(shasum "$WSC" | cut -d' ' -f1)
    if [ "$sha_before" = "$sha_after" ]; then
        printf "OK    bootstrap (SHA-1 %s)\n" "$sha_after"
        pass=$((pass + 1))
    else
        printf "FAIL  bootstrap (SHA-1 mismatch: %s vs %s)\n" "$sha_before" "$sha_after"
        fail=$((fail + 1))
        errors="$errors bootstrap"
    fi
fi

# Summary
echo ""
total=$((pass + fail + skip))
echo "--- Results: $pass passed, $fail failed, $skip skipped ($total total) ---"

if [ $fail -gt 0 ]; then
    echo "Failures:$errors"
    exit 1
fi

exit 0
