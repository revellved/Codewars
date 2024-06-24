# frozen_string_literal: true

describe 'Solution' do
  it 'Fixed tests' do
    extend TwoSum

    Test.assert_equals(two_sum([1, 2, 3], 4).sort, [0, 2])
    Test.assert_equals(two_sum([1234, 5678, 9012], 14690).sort, [1, 2])
    Test.assert_equals(two_sum([2, 2, 3], 4).sort, [0, 1])
  end
end

