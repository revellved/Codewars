# frozen_string_literal: true

describe 'Isograms spec solution' do
  include Isograms

  it 'Fixed tests' do
    Test.assert_equals(isogram?('Dermatoglyphics'), true)
    Test.assert_equals(isogram?('isogram'), true)
    Test.assert_equals(isogram?('aba'), false, 'same chars may not be adjacent')
    Test.assert_equals(isogram?('moOse'), false, 'same chars may not be same case')
    Test.assert_equals(isogram?('isIsogram'), false)
    Test.assert_equals(isogram?(''), true, 'an empty string is a valid isogram')
  end
end
