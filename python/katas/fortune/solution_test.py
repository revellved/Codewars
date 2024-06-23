import codewars_test as test
from katas.fortune.solution import fortune

@test.describe('Tests')
def fixed_tests():
        
    @test.it('Basic Tests')
    def tests():
        test.assert_equals(fortune(100000, 1, 2000, 15, 1), True)
        test.assert_equals(fortune(100000, 1, 9185, 12, 1), False)
        test.assert_equals(fortune(100000000, 1, 100000, 50, 1), True)
        test.assert_equals(fortune(100000000, 1.5, 10000000, 50, 1), False)
        test.assert_equals(fortune(100000000, 5, 1000000, 50, 1), True)
        
