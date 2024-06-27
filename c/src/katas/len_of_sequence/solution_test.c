#include "solution.c"

#include <criterion/criterion.h>
#include <stdio.h>

extern size_t sequence_length (size_t n, const int numbers[n], int elem);

#define ARR_LEN(array) (sizeof(array) / sizeof *(array))

#define fixed_test(numbers, elem, exp) do_test(ARR_LEN(numbers), numbers, elem, exp)

static void do_test (size_t n, const int numbers[n], int elem, size_t expected);

Test(tests_suite, sample_tests)
{
    do_test(0, NULL,                     1, 0);
    fixed_test(((int[]){1, 0, 2, 2, 1}), 1, 5);
    fixed_test(((int[]){0, 1, 2, 1, 0}), 1, 3);
    fixed_test(((int[]){0, 9, 1, 1, 8}), 1, 2);
    fixed_test(((int[]){1, 5, 1, 0, 1}), 1, 0);
    fixed_test(((int[]){0, 0, 0, 2, 6}), 1, 0);
    fixed_test(((int[]){0, 4, 1, 8, 6}), 1, 0);
}

static void print_array (size_t length, const int array[length])
{
    printf("{");
    for (size_t i = 0; i < length; i++)
        printf("%d%s", array[i], (i == length - 1) ? "" : ", ");
    printf("}");
}

static void do_test (size_t n, const int numbers[n], int elem, size_t expected)
{
    size_t actual = sequence_length(n, numbers, elem);

    if (actual != expected) {
        printf("array = ");
        print_array(n, numbers);
        fflush(stdout);
    }
    cr_assert_eq(actual, expected,
        "expected %zu, but got %zu, for element %d",
        expected, actual, elem
    );
}
