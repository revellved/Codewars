#include "solution.c"
#include <criterion/criterion.h>
#include <limits.h>

static void do_test(unsigned n, unsigned long long expected);

Test(test_suite, sample_tests) {
  do_test(3212u, 9414ull);
  do_test(2112u, 4114ull);
  do_test(0u, 0ull);
  do_test(999u, 818181ull);
  do_test(10001u, 10001ull);
  do_test(3210987654u, 9410816449362516ull);
  do_test(3999999999u, 9818181818181818181ull); // :p
  do_test(UINT_MAX, 164811681364948125ull);
}

extern unsigned long long square_digits(unsigned n);

static void do_test(unsigned n, unsigned long long expected) {
  unsigned long long actual = square_digits(n);
  cr_assert_eq(actual, expected, "for n = %u, expected %llu, but got %llu", n,
               expected, actual);
}
