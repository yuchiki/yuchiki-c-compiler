#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    RUST_BACKTRACE=1 cargo run -- "$input" > tmpdir/tmp.s
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
assert 21 "5+20-4"
assert 7 "1+2*3"
assert 5 "3+10/5"
assert 14 "2*(3+4)"
assert 10 "-10+20"

assert 1 "1 == 1"
assert 0 "1 == 2"

assert 1 "1 != 2"
assert 0 "1 != 1"

assert 1 "1 < 2"
assert 0 "1 < 1"
assert 0 "2 < 1"

assert 1 "1 <= 2"
assert 1 "1 <= 1"
assert 0 "2 <= 1"

assert 1 "2 > 1"
assert 0 "1 > 1"
assert 0 "1 > 2"

assert 1 "2 >= 1"
assert 1 "1 >= 1"
assert 0 "1 >= 2"

echo OK
