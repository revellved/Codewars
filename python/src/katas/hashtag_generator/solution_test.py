import codewars_test as test
from . solution import generate_hashtag

@test.describe("Hastag Generator")
def tests():
    @test.it("Should generate the correct hashtag in fixed tests")
    def _():
        test.assert_equals(
            generate_hashtag('Codewars'),
            '#Codewars',
            'Should handle a single word.'
        )
        
        test.assert_equals(
            generate_hashtag('Codewars      '),
            '#Codewars',
            'Should handle trailing whitespace.'
        )
        
        test.assert_equals(
            generate_hashtag('      Codewars'),
            '#Codewars',
            'Should handle leading whitespace.'
        )
        
        test.assert_equals(
            generate_hashtag('Codewars Is Nice'),
            '#CodewarsIsNice',
            'Should remove spaces.'
        )
        
        test.assert_equals(
            generate_hashtag('codewars is nice'),
            '#CodewarsIsNice',
            'Should capitalize first letters of words.'
        )
        
        test.assert_equals(
            generate_hashtag('CoDeWaRs is niCe'),
            '#CodewarsIsNice',
            'Only the first letter of each word should be capitalized in the final hashtag,\
            all other letters must be lower case.'
        )
        
        test.assert_equals(
            generate_hashtag('c i n'),
            '#CIN',
            'A single letter is considered to be a word of length 1,\
            so should capitalize first letters of words of length 1.'
        )
        
        test.assert_equals(
            generate_hashtag('codewars  is  nice'),
            '#CodewarsIsNice',
            'Should deal with unnecessary middle spaces.'
        )
        
    @test.it("Should return False if the input is empty, or result is longer than 140 chars")
    def _():
        test.assert_equals(
            generate_hashtag(''),
            False,
            'Expected an empty string to return False'
        )
        
        test.assert_equals(
            generate_hashtag(
                'Loooooooooooooooooooooooooooooooooooooooooooooooooooooo\
                ooooooooooooooooooooooooooooooooooooooooooooooooooooooooo\
                oooooooooooooooooooooooooooooooooooooooooooong Cat'),
            False,
            'Should return False if the final string is longer than 140 chars.'
        )
