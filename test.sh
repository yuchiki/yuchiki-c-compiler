#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    cargo run -- "$input" > tmpdir/tmp.s
    cc -o tmpdir/tmp tmpdir/tmp.s
    ./tmpdir/tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42
echo OK
