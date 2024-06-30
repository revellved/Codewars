#include <stdlib.h>
#include <ctype.h>
#include <string.h>

char *camel_case(const char *s) {
    int n = strlen(s) + 1, j = 0;
    char *new = malloc(n * sizeof(char));
    
    for (int i = 0; i < n; ++i) {
        if (!isspace(s[i])) {
            if (i == 0)
                new[j] = toupper(s[i]);
            else if (isspace(s[i-1]))
                new[j] = toupper(s[i]);
            else
                new[j] = s[i];
            j++;
        }
    }
    return new;
}
