# frozen_string_literal: true

describe 'Solution' do
  it 'Fixed tests' do
    extend RomanNumeralsDecoder

    Test.assert_equals(solution('XXI'), 21)
    Test.assert_equals(solution('I'), 1)
    Test.assert_equals(solution('IV'), 4)
    Test.assert_equals(solution('MMVIII'), 2008)
    Test.assert_equals(solution('MDCLXVI'), 1666)
  end
end
