# frozen_string_literal: true

# https://www.codewars.com/kata/51b62bf6a9c58071c600001b/ruby
module RomanNumeralsEncoder
  # @return [Hash<String, Integer>]
  def numerals
    {
      'M' => 1000, 'CM' => 900, 'D' => 500, 'CD' => 400, 'C' => 100,
      'XC' => 90, 'L' => 50, 'XL' => 40, 'X' => 10,
      'IX' => 9, 'V' => 5, 'IV' => 4, 'I' => 1
    }
  end

  # @param number [Integer]
  # @return [String]
  def solution(number)
    return '' if number <= 0

    numerals.each { |key, val| return key + solution(number - val) if number >= val }
  end
end
