# frozen_string_literal: true

# https://www.codewars.com/kata/5467e4d82edf8bbf40000155/ruby
module DescendingOrder
  extend T::Sig

  sig { params(num: Integer).returns(Integer) }
  def descending_order(num)
    num.to_s.chars.sort.reverse.join.to_i
  end
end
