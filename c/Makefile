
build:
	clang ./src/main.c -o ./bin/main

run:
	make build && ./bin/main

test_build:
	clang ./src/katas/**/*_test.c -o ./bin/test -lcriterion -lm -Wno-string-compare

test:
	make test_build && ./bin/test
