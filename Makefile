CFLAGS=-g -static


.PHONY: test test-docker clean
test-local:
	cargo test

test-docker:
	docker build --progress plain -t yuchiki-c-compiler-test -f tests/Dockerfile .
	docker run yuchiki-c-compiler-test



clean:
	rm -f tmpdir/tmp*
