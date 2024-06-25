import codewars_test as test
from . solution import dont_give_me_five

@test.describe("Dont Give Me Five")
def fixed_tests():
    @test.it('Basic Test Cases')
    def _():
        test.assert_equals(dont_give_me_five(1,9), 8)
        test.assert_equals(dont_give_me_five(4,17), 12)
