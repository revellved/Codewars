# frozen_string_literal: true

# https://www.codewars.com/kata/5813d19765d81c592200001a/ruby
module DontGiveMeFive
  # @param start [Integer]
  # @param end [Integer]
  # @return [Integer]
  def dont_give_me_five(start, ends)
    ends - (ends / 5 - ends / 10 - start / 5).floor - start + 1
  end
end
