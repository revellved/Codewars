#include "solution.cpp"
#include <string>
#include <vector>

std::vector<std::string> number(const std::vector<std::string> &lines);

void do_test(const std::vector<std::string> &lines,
             const std::vector<std::string> &expected) {
  Assert::That(number(lines), EqualsContainer(expected));
}

Describe(sample_test){It(example_tests){do_test({}, {});
do_test({"a", "b", "c"}, {"1: a", "2: b", "3: c"});
}
}
;
