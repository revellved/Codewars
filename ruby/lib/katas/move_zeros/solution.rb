# frozen_string_literal: true

# https://www.codewars.com/kata/52597aa56021e91c93000cb0/ruby
module MoveZeros
  extend T::Sig

  sig { params(arr: T::Array[Integer]).returns(T::Array[Integer]) }
  def moveZeros(arr) 
    arr.sort_by { |w| w == 0 ? 1 : 0 }
  end
end
