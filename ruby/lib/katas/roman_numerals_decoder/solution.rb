# frozen_string_literal: true

# https://www.codewars.com/kata/51b6249c4612257ac0000005/ruby
module RomanNumeralsDecoder
  extend T::Sig

  def dict
    {
      'I' => 1,
      'V' => 5,
      'X' => 10,
      'L' => 50,
      'C' => 100,
      'D' => 500,
      'M' => 1_000
    }
  end

  sig { params(roman: String).returns(Integer) }
  def solution(roman)
    last = 0
    roman.chars.reverse.reduce(0) do |acc, roman_num|
      acc += dict[roman_num] < last ? -dict[roman_num] : dict[roman_num]
      last = dict[roman_num]
      acc
    end
  end
end
