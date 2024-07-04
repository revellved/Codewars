# frozen_string_literal: true

# https://www.codewars.com/kata/52c31f8e6605bcc646000082/ruby
module TwoSum
  # @param numbers [Array<Integer>]
  # @return [Array<Integer>]
  def two_sum(numbers, target)
    numbers.each_with_index.to_a.combination(2) do |arr|
      return [arr[0][1], arr[1][1]] if arr[0][0] + arr[1][0] == target
    end
  end
end
