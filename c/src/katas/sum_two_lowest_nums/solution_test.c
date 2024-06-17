#include "solution.c"
#include <criterion/criterion.h>

void tester(size_t n, const int numbers[n], long expected) {
  long actual = sum_two_smallest_numbers(n, numbers);
  cr_assert_eq(actual, expected, "expected %lu, but got %lu", expected, actual);
};

Test(Example_Test, should_pass_the_test_provided) {
  {
    const int numbers[5] = {19, 5, 42, 2, 77};
    tester(5, numbers, 7);
  }
  {
    const int numbers[5] = {5, 8, 12, 19, 22};
    tester(5, numbers, 13);
  }
}
