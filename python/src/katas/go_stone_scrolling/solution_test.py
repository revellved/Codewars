import codewars_test as test
from . solution import determine_winner

@test.describe("6x6 Boards")
def test_group():
    @test.it("6x6 Board 1")
    def _():
        test.assert_equals(determine_winner(
            [["W","W","W","B","B","B"], 
             ["W","W","W","W","B","B"], 
             ["W","W","W","B","B","B"], 
             ["W","X","W","B","B","B"], 
             ["X","W","B","B","B","B"], 
             ["W","W","B","X","B","X"]]
        ), ("B", 17))
