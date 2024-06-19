# frozen_string_literal: true

# https://www.codewars.com/kata/52b757663a95b11b3d00062d/ruby
module WeirdStringCase
  extend T::Sig

  sig { params(string: String).returns(String) }
  def weirdcase(string)
    string.gsub(/\w{1,2}/) { ::Regexp.last_match(0).capitalize }
  end
end
