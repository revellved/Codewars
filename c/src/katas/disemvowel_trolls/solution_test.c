#include "solution.c"

#include <criterion/criterion.h>
#include <stdlib.h>

static void do_test (const char *input, const char *expected);

Test(tests_suite, fixed_tests) 
{
  do_test("This website is for losers LOL!", "Ths wbst s fr lsrs LL!");
  do_test("No offense but,\nYour writing is among the worst I've ever read", "N ffns bt,\nYr wrtng s mng th wrst 'v vr rd");
  do_test("What are you, a communist?", "Wht r y,  cmmnst?");
  do_test("Yyy!", "Yyy!");
}

extern char *disemvowel(const char *str);

static void do_test (const char *input, const char *expected)
{
	char *actual = disemvowel(input);
	cr_assert_str_eq(actual, expected,
		"input:    \"%s\"\nexpected: \"%s\"\nactual:    \"%s\"",
		input, expected, actual
	);
	free(actual);
}
