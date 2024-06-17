#include <ctype.h>
#include <stdbool.h>

bool xo(const char *str) {
  unsigned x = 0, o = 0;
  for (const char *p = str; *p; p++) {
    if (tolower(*p) == 'x')
      x++;
    else if (tolower(*p) == 'o')
      o++;
  }
  return x == o;
}
