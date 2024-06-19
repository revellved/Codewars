# frozen_string_literal: true

# https://www.codewars.com/kata/5467e4d82edf8bbf40000155/ruby
module NumberPrime
  extend T::Sig

  sig { params(num: Integer).returns(T::Boolean) }
  def prime?(num)
    return false if num <= 1

    (2..Math.sqrt(num)).none? { |i| (num % i).zero? }
  end
end
