# frozen_string_literal: true

describe 'Dont Give Me Five spec solution' do
  include DontGiveMeFive

  it 'Fixed tests' do
    Test.assert_equals(dont_give_me_five(1, 9), 8)
    Test.assert_equals(dont_give_me_five(4, 17), 12)
  end
end
