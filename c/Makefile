include ../Makefile

LIB_FLAGS_TEST=-lcriterion -lm -Wno-string-compare
BUILD_OPTIONS_TEST=./src/katas/**/*_test.c -o ./bin/test $(LIB_FLAGS_TEST)
BUILD_OPTIONS=./src/main.c -o ./bin/main

prepare:
	mkdir -p ./bin

## -- CLANG
clang_build:
	make prepare
	clang $(BUILD_OPTIONS)

clang_run:
	make clang_build && ./bin/main

clang_test_build:
	make prepare
	clang $(BUILD_OPTIONS_TEST)

clang_test:
	make clang_test_build && ./bin/test


## -- GCC
gcc_build:
	make prepare
	gcc $(BUILD_OPTIONS)

gcc_run:
	make gcc_build && ./bin/main

gcc_test_build:
	make prepare
	gcc $(BUILD_OPTIONS_TEST)

gcc_test:
	make gcc_test_build && ./bin/test


## -- CURRENT
build:
	make gcc_build

run:
	make gcc_run

test:
	make gcc_test
