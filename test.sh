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


    assert 0 "main(){0;}"
    assert 42 "main(){42;}"
    assert 21 "main(){5+20-4;}"
    assert 7 "main(){1+2*3;}"
    assert 5 "main(){3+10/5;}"
    assert 14 "main(){2*(3+4);}"
    assert 10 "main(){-10+20;}"

    assert 1 "main(){1 == 1;}"
    assert 0 "main(){1 == 2;}"

    assert 1 "main(){1 != 2;}"
    assert 0 "main(){1 != 1;}"

    assert 1 "main(){1 < 2;}"
    assert 0 "main(){1 < 1;}"
    assert 0 "main(){2 < 1;}"

    assert 1 "main(){1 <= 2;}"
    assert 1 "main(){1 <= 1;}"
    assert 0 "main(){2 <= 1;}"

    assert 1 "main(){2 > 1;}"
    assert 0 "main(){1 > 1;}"
    assert 0 "main(){1 > 2;}"

    assert 1 "main(){2 >= 1;}"
    assert 1 "main(){1 >= 1;}"
    assert 0 "main(){1 >= 2;}"

    assert 10 "main(){1+1;3+7;}"

    assert 3 "main(){num=3;}"
    assert 5 "main(){num=3;num+2;}"
    assert 6 "main(){num1=num2=3; num1+num2;}"

    assert 3 "main(){return 3;}"
    assert 3 "main(){return 3; return 5;}"

    assert 3 "main(){if (1) 3;}"
    assert 0 "main(){if (0) 3;}" # 括弧の中が最後に評価されるので、0が返る

    assert 3 "main(){if (1) 3; else 5;}"
    assert 5 "main(){if (0) 3; else 5;}"

    assert 16 "main(){i = 1; while (i <= 10) i = i * 2; i;}"

    assert 55 "main(){sum = i = 0; for (i = 1; i <= 10; i = i + 1) sum = sum + i; sum;}"

    assert 55 "main(){i = sum = 0; while (i <= 10) { sum = sum + i; i = i + 1; } sum;}" # block のテスト

    assert 91 "main(){external_func(1,2,3,4,5,6);}"

    assert 49 "my_func(a, b, c, d, e, f){g = 7; h = a + b * 2 + c * 3 + d * 4 + e * 5 + f * 6 + g; return h / 2;} main(){my_func(1,2,3,4,5,6);}"

    echo OK
}

main
