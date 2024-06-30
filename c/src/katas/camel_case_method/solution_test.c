#include "solution.c"

#include <criterion/criterion.h>
#include <stdlib.h>

char *camel_case(const char *s);

Test(Sample_Test, should_return_the_camel_case_string)
{
    char *s;

    cr_assert_str_eq(s = camel_case("test case"), "TestCase");
    free(s);
    cr_assert_str_eq(s = camel_case("camel case method"), "CamelCaseMethod");
    free(s);
    cr_assert_str_eq(s = camel_case("say hello "), "SayHello");
    free(s);
    cr_assert_str_eq(s = camel_case(" camel case word"), "CamelCaseWord");
    free(s);
    cr_assert_str_eq(s = camel_case(""), "");
    free(s);
}
