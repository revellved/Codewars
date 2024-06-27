#include <stddef.h>

size_t sequence_length(size_t n, const int numbers[], int elem) {
    int count = 0;
    int startIndex = -1;
    int endIndex = -1;

    for (int i = 0; i < n; i++) {
        if (numbers[i] == elem) {
            if (startIndex == -1) {
                startIndex = i;
            } else {
                endIndex = i;
            }
            count++;
        }
    }

    if (count != 2) {
        return 0;
    }

    return endIndex - startIndex + 1;
}
