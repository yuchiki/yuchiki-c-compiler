CFLAGS=-g -static


test: src/main.rs
	./test.sh

.PHONY: test
