#include <stddef.h>

void move_zeros(size_t len, int arr[len]) {
    size_t index_source, index_dest; 
    index_dest = 0; 

    for (index_source = 0; index_source < len; index_source++) 
        if (arr[index_source] != 0) arr[index_dest++] = arr[index_source]; 
  
    while (index_dest < len)
        arr[index_dest++] = 0;
}
