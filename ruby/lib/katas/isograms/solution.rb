# frozen_string_literal: true

# https://www.codewars.com/kata/54ba84be607a92aa900000f1/ruby
module Isograms
  # @param string [String]
  # @return [Boolean]
  def isogram?(string)
    !string.downcase.chars.uniq!
  end
end
