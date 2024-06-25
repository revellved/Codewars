import codewars_test as test
from . solution import dont_give_me_five

@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(dont_give_me_five(1,9), 8)
        test.assert_equals(dont_give_me_five(4,17), 12)
