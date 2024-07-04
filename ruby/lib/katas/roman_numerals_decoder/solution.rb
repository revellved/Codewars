# frozen_string_literal: true

# https://www.codewars.com/kata/51b6249c4612257ac0000005/ruby
module RomanNumeralsDecoder
  # @return [Hash<String, Integer>]
  def roman_dict
    { 'M' => 1000, 'D' => 500, 'C' => 100, 'L' => 50, 'X' => 10, 'V' => 5, 'I' => 1 }
  end

  # @param roman [String]
  # @return [Integer]
  def solution(roman)
    roman.chars.map { |c| roman_dict[c] }.reduce { |a, b| a < b ? b - a : a + b }
  end
end
