#include <stdbool.h>

bool xo(const char *str) {
  unsigned int count_x = 0, count_o = 0;
  while (*str)
    switch (*str++) {
    case 'x':
    case 'X':
      count_x++;
      break;
    case 'o':
    case 'O':
      count_o++;
    };

  return count_o == count_x;
}
