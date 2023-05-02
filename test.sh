#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    RUST_BACKTRACE=1 cargo run -- "$input" > tmpdir/tmp.s
    cc --static -o tmpdir/tmp tmpdir/tmp.s tmpdir/external_func.s
    ./tmpdir/tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}


pre_test() {
    cc --static -S -o tmpdir/external_func.s tmpdir/external_func.c
}

main() {
    pre_test


    assert 0 "0;"
    assert 42 "42;"
    assert 21 "5+20-4;"
    assert 7 "1+2*3;"
    assert 5 "3+10/5;"
    assert 14 "2*(3+4);"
    assert 10 "-10+20;"

    assert 1 "1 == 1;"
    assert 0 "1 == 2;"

    assert 1 "1 != 2;"
    assert 0 "1 != 1;"

    assert 1 "1 < 2;"
    assert 0 "1 < 1;"
    assert 0 "2 < 1;"

    assert 1 "1 <= 2;"
    assert 1 "1 <= 1;"
    assert 0 "2 <= 1;"

    assert 1 "2 > 1;"
    assert 0 "1 > 1;"
    assert 0 "1 > 2;"

    assert 1 "2 >= 1;"
    assert 1 "1 >= 1;"
    assert 0 "1 >= 2;"

    assert 10 "1+1;3+7;"

    assert 3 "num=3;"
    assert 5 "num=3;num+2;"
    assert 6 "num1=num2=3; num1+num2;"

    assert 3 "return 3;"
    assert 3 "return 3; return 5;"

    assert 3 "if (1) 3;"
    assert 0 "if (0) 3;" # 括弧の中が最後に評価されるので、0が返る

    assert 3 "if (1) 3; else 5;"
    assert 5 "if (0) 3; else 5;"

    assert 16 "i = 1; while (i <= 10) i = i * 2; i;"

    assert 55 "sum = i = 0; for (i = 1; i <= 10; i = i + 1) sum = sum + i; sum;"

    assert 55 "i = sum = 0; while (i <= 10) { sum = sum + i; i = i + 1; } sum;" # block のテスト

    assert 91 "external_func(1,2,3,4,5,6);"

    echo OK
}

main
