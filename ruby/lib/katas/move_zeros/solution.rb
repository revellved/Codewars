# frozen_string_literal: true

# https://www.codewars.com/kata/52597aa56021e91c93000cb0/ruby
module MoveZeros
  # @param arr [Array<Integer>]
  # @return [Array<Integer>]
  def moveZeros(arr) 
    arr.sort_by { |w| w == 0 ? 1 : 0 }
  end
end
