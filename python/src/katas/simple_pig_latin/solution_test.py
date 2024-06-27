import codewars_test as test
from . solution import pig_it

@test.describe("Simple Pig Latin")
def fixed_tests():
    @test.it('Basic Test Cases')
    def _():
        test.assert_equals(pig_it('Pig latin is cool'),'igPay atinlay siay oolcay')
        test.assert_equals(pig_it('This is my string'),'hisTay siay ymay tringsay')
