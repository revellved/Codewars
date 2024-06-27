#include "solution.c"

#include <criterion/criterion.h>

static void do_test(const int triplet[3], int expected);

Test(find_middle_el_test, sample_tests)
{
	do_test((int[3]){2, 1, 3}, 0);
	do_test((int[3]){2, 3, 1}, 0);
	do_test((int[3]){1, 2, 3}, 1);
	do_test((int[3]){3, 2, 1}, 1);
	do_test((int[3]){1, 3, 2}, 2);
	do_test((int[3]){3, 1, 2}, 2);
}

extern int gimme(const int triplet[3]);

static void do_test(const int triplet[3], int expected) {
  int actual = gimme(triplet);
  cr_assert_eq(actual, expected, "expected %i, but got %i", expected, actual);
}
