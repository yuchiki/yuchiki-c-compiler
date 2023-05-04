CFLAGS=-g -static


.PHONY: test
test:
	cargo test

clean :
	rm -f tmpdir/tmp*
