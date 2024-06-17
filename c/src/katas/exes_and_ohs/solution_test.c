#include "solution.c"
#include <criterion/criterion.h>
#include <stdbool.h>

bool xo(const char *str);

Test(xo_test, should_pass_all_the_tests_provided) {
  cr_assert_eq(xo("xo"), true);
  cr_assert_eq(xo("Xo"), true);
  cr_assert_eq(xo("xxOo"), true);
  cr_assert_eq(xo("xxxm"), false);
  cr_assert_eq(xo("Oo"), false);
  cr_assert_eq(xo("ooom"), false);
}
