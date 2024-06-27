include ../Makefile

clang_build:
	mkdir -p ./bin
	clang ./src/main.c -o ./bin/main

clang_run:
	make clang_build && ./bin/main

clang_test_build:
	clang ./src/katas/**/*_test.c -o ./bin/test -lcriterion -lm -Wno-string-compare

clang_test:
	make clang_test_build && ./bin/test


gcc_build:
	mkdir -p ./bin
	gcc ./src/main.c -o ./bin/main

gcc_run:
	make gcc_build && ./bin/main

gcc_test_build:
	gcc ./src/katas/**/*_test.c -o ./bin/test -lcriterion -lm -Wno-string-compare

gcc_test:
	make gcc_test_build && ./bin/test


build:
	make gcc_build

run:
	make gcc_run

test:
	make gcc_test
