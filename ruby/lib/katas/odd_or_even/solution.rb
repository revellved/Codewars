# frozen_string_literal: true

# https://www.codewars.com/kata/5949481f86420f59480000e7/ruby
module OddOrEven
  # @param array [Array<Integer>]
  # @return [String]
  def odd_or_even(array)
    array.sum.odd? ? 'odd' : 'even'
  end
end
