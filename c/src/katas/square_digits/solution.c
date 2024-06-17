#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

unsigned long long square_digits(unsigned n) {
  char str_sum[50] = "";
  while (n != 0) {
    unsigned num = pow(n % 10, 2);
    char temp_str[50];
    sprintf(temp_str, "%d", num);
    strcat(temp_str, str_sum);
    sprintf(str_sum, "%s", temp_str);

    n /= 10;
  }
  return strtoull(str_sum, NULL, 10);
}
