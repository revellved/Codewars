# frozen_string_literal: true

# https://www.codewars.com/kata/554ca54ffa7d91b236000023/ruby
module DeleteNth
  # @param order [Array<Integer>]
  # @param max_e [Integer]
  # @return [Array<Integer>]
  def delete_nth(order, max_e)
    occurrences = Hash.new(0)
    order.reject { |item| (occurrences[item] += 1) > max_e }
  end
end
