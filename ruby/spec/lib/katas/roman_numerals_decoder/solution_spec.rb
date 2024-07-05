# frozen_string_literal: true

describe 'Roman Numerals Decoder spec solution' do
  include RomanNumeralsDecoder

  it 'Fixed tests' do
    Test.assert_equals(solution('XXI'), 21)
    Test.assert_equals(solution('I'), 1)
    Test.assert_equals(solution('IV'), 4)
    Test.assert_equals(solution('MMVIII'), 2008)
    Test.assert_equals(solution('MDCLXVI'), 1666)
  end
end
