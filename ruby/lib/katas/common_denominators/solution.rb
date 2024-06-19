# frozen_string_literal: true

# https://www.codewars.com/kata/54d7660d2daf68c619000d95/ruby
module CommonDeniminators
  extend T::Sig

  sig { params(number: T::Array[[Integer, Integer]]).returns(T::Array[[Integer, Integer]]) }
  def convert_fracts(lst)
    denominators = lst.map { |frac| frac[1] }
    lcm = denominators.reduce(1, :lcm)

    lst.map { |frac| [frac[0] * (lcm / frac[1]), lcm] }
  end
end
