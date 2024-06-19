# frozen_string_literal: true

# https://www.codewars.com/kata/5277c8a221e209d3f6000b56/ruby
module ValidBraces
  extend T::Sig

  def braces_dict
    { '(' => ')', '{' => '}', '[' => ']' }
  end

  sig { params(braces: String).returns(T::Boolean) }
  def valid_braces(braces)
    stack = []

    braces.chars.each do |char|
      if braces_dict.key?(char)
        stack.push(char)
      elsif braces_dict.value?(char)
        return false if stack.empty? || braces_dict[stack.pop] != char
      end
    end

    stack.empty?
  end
end
