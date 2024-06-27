import codewars_test as test
from . solution import tower_builder

@test.describe("Build Tower")
def fixed_tests():
    @test.it('Basic Test Cases')
    def _():
        test.assert_equals(tower_builder(1), ['*', ])
        test.assert_equals(tower_builder(2), [' * ', '***'])
        test.assert_equals(tower_builder(3), ['  *  ', ' *** ', '*****'])
