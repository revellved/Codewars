import codewars_test as test
import katas.go_stone_scrolling.solution as solution

@test.describe("6x6 Boards")
def test_group():
    @test.it("6x6 Board 1")
    def test_case():
        test.assert_equals(solution.determine_winner([["W","W","W","B","B","B"], ["W","W","W","W","B","B"], ["W","W","W","B","B","B"], ["W","X","W","B","B","B"], ["X","W","B","B","B","B"], ["W","W","B","X","B","X"]]), ("B", 17))
