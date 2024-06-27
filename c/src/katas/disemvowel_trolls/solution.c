#include <stdlib.h>
#include <string.h>

char *disemvowel(const char *s)
{
  char * ret = calloc(strlen(s) + 1, 1);
  for (char *q = ret; *s; s++) if (!strchr("aeiouAEIOU", *s)) *q++ = *s;
  return ret;
}
