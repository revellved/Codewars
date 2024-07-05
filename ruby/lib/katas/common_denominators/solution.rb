# frozen_string_literal: true

# https://www.codewars.com/kata/54d7660d2daf68c619000d95/ruby
module CommonDenominators
  # @param lst [Array<Array<Integer>>]
  # @return [Array<Array<Integer>>]
  def convert_fracts(lst)
    denominators = lst.map { |frac| frac[0] }
    lcm = denominators.reduce(1, :lcm)

    lst.map { |frac| [frac[0] * (lcm / frac[1]), lcm] }
  end
end
