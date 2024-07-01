#include "solution.c"

#include <criterion/criterion.h>
#include <stdio.h>
#include <string.h>

void move_zeros(size_t len, int arr[len]);

char* print_array(char *arr_str, int* data, int len){
    char tmp_str[3];
    
    arr_str[0] = 0;
    strcat(arr_str, "[");
    for(size_t i = 0; i < len; i++ ) {
        sprintf(tmp_str, "%d%s", data[i], (i<len-1) ? ", " : "");
        strcat(arr_str, tmp_str);    
    }
    strcat(arr_str, "]");

    return arr_str;
}

static void do_test(size_t len, int *arr, int *expected) {
  char arr1_str[len*3], arr2_str[len*3];

  move_zeros(8, arr);
  cr_assert_eq(arr, expected, "expected: %s, got %s", print_array(arr1_str, expected, len), print_array(arr2_str, arr, len));
}

Test(move_zeros, sample_tests) {
    do_test(8, 
            ((int []) {0, 1, 0, 2, 0, 3, 4, 5}),
						((int []) {1, 2, 3, 4, 5, 0, 0, 0}));

    do_test(20, 
            ((int []) {9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9}),
						((int []) {9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}));
  
    do_test(2, 
            ((int []) {0, 0}),
						((int []) {0, 0}));

    do_test(1, 
            ((int []) {0}),
						((int []) {0}));

    do_test(0, 
            ((int []) {}),
						((int []) {}));
}
