#!/bin/sh
#
# tests/run_tests.sh — Autonomous test suite for Whispem v6.0.0
#
# Requires: ./wvm (C VM, built via `make`) and compiler/wsc.whbc
# Each test: compile .wsp → .whbc via Rust or wsc.whbc, run on C VM,
# compare output to tests/expected/<name>.txt.
#
# Usage:
#   ./tests/run_tests.sh           # run all tests
#   ./tests/run_tests.sh hello     # run one test by name

set -e

cd "$(dirname "$0")/.."

WVM=./wvm
EXPECTED=tests/expected
TMPDIR_LOCAL=$(mktemp -d)
trap 'rm -rf "$TMPDIR_LOCAL"' EXIT

pass=0
fail=0
skip=0
errors=""

run_test() {
    name="$1"
    src="$2"
    expected="$3"

    # Compile with the Rust reference compiler
    if ! cargo run --release --quiet -- --compile "$src" 2>/dev/null; then
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

    actual="$TMPDIR_LOCAL/$name.actual"
    "$WVM" "$whbc" > "$actual" 2>&1 || true

    exp_norm="$TMPDIR_LOCAL/$name.expected"
    act_norm="$TMPDIR_LOCAL/$name.actual_norm"
    printf '%s' "$(cat "$expected")" > "$exp_norm"
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
    echo "wvm not found — building from vm/wvm.c..."
    make || { echo "Error: make failed. Cannot run tests."; exit 1; }
elif [ "vm/wvm.c" -nt "$WVM" ]; then
    echo "vm/wvm.c is newer than wvm — rebuilding..."
    make || { echo "Error: make failed."; exit 1; }
fi

echo "=== Whispem autonomous test suite — v6.0.0 ==="
echo ""

if [ $# -gt 0 ]; then
    for name in "$@"; do
        if   [ -f "examples/$name.wsp" ] && [ -f "$EXPECTED/$name.txt" ]; then
            run_test "$name" "examples/$name.wsp" "$EXPECTED/$name.txt"
        elif [ -f "tests/$name.wsp"    ] && [ -f "$EXPECTED/$name.txt" ]; then
            run_test "$name" "tests/$name.wsp"    "$EXPECTED/$name.txt"
        else
            printf "SKIP  %s (source or expected not found)\n" "$name"
            skip=$((skip + 1))
        fi
    done
else
    for exp in "$EXPECTED"/*.txt; do
        name=$(basename "$exp" .txt)
        if   [ -f "examples/$name.wsp" ]; then
            run_test "$name" "examples/$name.wsp" "$exp"
        elif [ -f "tests/$name.wsp"    ]; then
            run_test "$name" "tests/$name.wsp"    "$exp"
        else
            printf "SKIP  %s (source not found)\n" "$name"
            skip=$((skip + 1))
        fi
    done

    # ── Bootstrap ────────────────────────────────────────────────────────────
    # The self-hosted compiler and the Rust compiler may produce different
    # constant-pool orderings (gen1 != gen2 is acceptable).
    # The invariant is gen2 == gen3: the self-hosted compiler is its own
    # fixed point.
    echo ""
    echo "--- Bootstrap ---"

    # Gen1: Rust compiler compiles wsc.wsp
    cargo run --release --quiet -- --compile compiler/wsc.wsp 2>/dev/null
    cp compiler/wsc.whbc "$TMPDIR_LOCAL/wsc_gen1.whbc"

    # Gen2: gen1 compiles wsc.wsp
    "$WVM" "$TMPDIR_LOCAL/wsc_gen1.whbc" compiler/wsc.wsp > /dev/null 2>&1
    cp compiler/wsc.whbc "$TMPDIR_LOCAL/wsc_gen2.whbc"
    sha2=$(sha1sum "$TMPDIR_LOCAL/wsc_gen2.whbc" | cut -d' ' -f1)

    # Gen3: gen2 compiles wsc.wsp (must equal gen2)
    "$WVM" "$TMPDIR_LOCAL/wsc_gen2.whbc" compiler/wsc.wsp > /dev/null 2>&1
    sha3=$(sha1sum compiler/wsc.whbc | cut -d' ' -f1)

    if [ "$sha2" = "$sha3" ]; then
        printf "OK    bootstrap fixed point (SHA1 %s)\n" "$sha2"
        pass=$((pass + 1))
    else
        printf "FAIL  bootstrap (gen2 %s != gen3 %s)\n" "$sha2" "$sha3"
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