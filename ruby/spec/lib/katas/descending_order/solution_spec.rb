# frozen_string_literal: true

describe 'Solution' do
  it 'Fixed tests' do
    extend DescendingOrder

    Test.assert_equals(descending_order(0), 0)
    Test.assert_equals(descending_order(1), 1)
    Test.assert_equals(descending_order(123_456_789), 987_654_321)
  end
end
