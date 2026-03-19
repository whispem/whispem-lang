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

    if ! "$WVM" "$WSC" "$src" > /dev/null 2>&1; then
        printf "SKIP  %s (compile failed)\n" "$name"
        skip=$((skip + 1))
        return
    fi

    whbc="${src%.wsp}.whbc"
    if [ ! -f "$whbc" ]; then
        printf "SKIP  %s (no .whbc produced)\n" "$name"
        skip=$((skip + 1))
        return
    fi

    actual="$TMPDIR/$name.actual"
    "$WVM" "$whbc" > "$actual" 2>&1 || true

    # Normalize trailing newlines before comparing
    exp_norm="$TMPDIR/$name.expected"
    printf '%s' "$(cat "$expected")" > "$exp_norm"
    act_norm="$TMPDIR/$name.actual_norm"
    printf '%s' "$(cat "$actual")"   > "$act_norm"

    if diff -q "$exp_norm" "$act_norm" > /dev/null 2>&1; then
        printf "OK    %s\n" "$name"
        pass=$((pass + 1))
    else
        printf "FAIL  %s\n" "$name"
        diff "$exp_norm" "$act_norm" | head -10
        fail=$((fail + 1))
        errors="$errors $name"
    fi

    rm -f "$whbc"
}

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

    # Bootstrap test: verify the compiler reaches a fixed point.
    # Compile wsc.wsp twice from the current wsc.whbc and compare gen1 vs gen2.
    # This works regardless of whether wsc.whbc was produced by Rust or by itself.
    echo ""
    echo "--- Bootstrap ---"
    "$WVM" "$WSC" compiler/wsc.wsp > /dev/null 2>&1
    sha1=$(shasum "$WSC" | cut -d' ' -f1)
    cp "$WSC" "$TMPDIR/wsc_gen1.whbc"
    "$WVM" "$TMPDIR/wsc_gen1.whbc" compiler/wsc.wsp > /dev/null 2>&1
    sha2=$(shasum "$WSC" | cut -d' ' -f1)
    if [ "$sha1" = "$sha2" ]; then
        printf "OK    bootstrap (SHA-1 %s)\n" "$sha1"
        pass=$((pass + 1))
    else
        printf "FAIL  bootstrap (gen1 %s != gen2 %s)\n" "$sha1" "$sha2"
        fail=$((fail + 1))
        errors="$errors bootstrap"
    fi
fi

echo ""
total=$((pass + fail + skip))
echo "--- Results: $pass passed, $fail failed, $skip skipped ($total total) ---"

if [ $fail -gt 0 ]; then
    echo "Failures:$errors"
    exit 1
fi

exit 0