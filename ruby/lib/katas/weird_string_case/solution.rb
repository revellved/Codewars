# frozen_string_literal: true

# https://www.codewars.com/kata/52b757663a95b11b3d00062d/ruby
module WeirdStringCase
  extend T::Sig

  sig { params(string: String).returns(String) }
  def weirdcase(string)
    string.split.map do |a|
      a.chars.each_slice(2).reduce('') do |acc, pairs|
        acc + pairs[0].to_s.upcase + pairs[1].to_s.downcase
      end
    end.join(' ')
  end
end
