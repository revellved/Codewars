# frozen_string_literal: true

# https://www.codewars.com/kata/51b62bf6a9c58071c600001b/train/ruby
module RomanNumeralsEncoder
  extend T::Sig

  sig { returns(T::Hash[String, Integer]) }
  def roman_dict
    {
      'M' => 1000, 'CM' => 900, 'D' => 500, 'CD' => 400, 'C' => 100,
      'XC' => 90, 'L' => 50, 'XL' => 40, 'X' => 10,
      'IX' => 9, 'V' => 5, 'IV' => 4, 'I' => 1
    }
  end

  sig { params(number: Integer).returns(String) }
  def solution(number)
    roman = ''
    roman_dict.map do |rom, num|
      roman += rom * (number / num)
      number %= num
    end
    roman
  end
end
