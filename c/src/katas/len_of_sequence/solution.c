#include <stddef.h>

#include <stddef.h>

size_t sequence_length (size_t n, const int numbers[n], int elem)
{
  int reps = 0;
  size_t size = 0;
  for (size_t i = 0; i < n; ++i) {
    if (numbers[i] == elem)
      ++reps;
    if (reps > 0)
      ++size;
    if (reps > 1 && numbers[i] != elem)
      --size;
  }
  return reps == 2 ? size: 0;
}
