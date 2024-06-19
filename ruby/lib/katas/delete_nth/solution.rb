# frozen_string_literal: true

# https://www.codewars.com/kata/554ca54ffa7d91b236000023/ruby
module DeleteNth
  extend T::Sig

  sig { params(order: T::Array[Integer], max_e: Integer).returns(T::Array[Integer]) }
  def delete_nth(order, max_e)
    hash = {}
    order.filter do |el|
      hash[el] = 0 if hash[el].nil?
      hash[el] += 1 and hash[el] <= max_e
    end
  end
end
