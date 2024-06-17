#include <limits.h>
#include <stddef.h>

long sum_two_smallest_numbers(size_t n, const int numbers[n]) {
  long n1 = LONG_MAX;
  long n2 = LONG_MAX;

  for (size_t i = 0; i < n; i++) {
    if (numbers[i] < n1) {
      if (n1 < n2) {
        n2 = n1;
      }
      n1 = numbers[i];
    } else if (numbers[i] < n2) {
      n2 = numbers[i];
    }
  }

  return n1 + n2;
}
