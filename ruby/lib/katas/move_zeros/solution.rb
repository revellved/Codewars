# frozen_string_literal: true

# https://www.codewars.com/kata/52597aa56021e91c93000cb0/ruby
module MoveZeros
  extend T::Sig

  sig { params(arr: T::Array[Integer]).returns(T::Array[Integer]) }
  def moveZeros(arr) 
    arr.reduce([[],[]]) {|acc, el| (el != 0) ? acc[0].push(el) : acc[1].push(el); acc }.flatten
  end
end
