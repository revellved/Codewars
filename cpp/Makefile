
build:
	clang++ -std=c++17 -pthread -lm -lcrypto -ldl -ltbb -lfmt -O2 -Wall -Wextra -fsanitize=return ./src/main.cpp -o ./bin/main

run:
	make build && ./bin/main

test_build:
	clang++ ./src/katas/**/*_test.cpp -o ./bin/test

test:
	make test_build && ./bin/test
