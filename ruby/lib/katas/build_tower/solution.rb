# frozen_string_literal: true

# https://www.codewars.com/kata/576757b1df89ecf5bd00073b/ruby
module BuildTower
  # @param num [Integer]
  # @return [String]
  def tower_builder(num)
    (1..num).map { |idx| ('*' * (idx * 2 - 1)).center(num * 2 - 1) }
  end
end
