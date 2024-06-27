#include <stdlib.h>
#include <string.h>

char* disemvowel(const char* str) {
    char* result = (char*)malloc(strlen(str) + 1);
    int j = 0;

    for (int i = 0; str[i] != '\0'; i++) {
        if (str[i] != 'a' && str[i] != 'e' && str[i] != 'i' && str[i] != 'o' && str[i] != 'u' &&
            str[i] != 'A' && str[i] != 'E' && str[i] != 'I' && str[i] != 'O' && str[i] != 'U') {
            result[j++] = str[i];
        }
    }

    result[j] = '\0';
    return result;
}
