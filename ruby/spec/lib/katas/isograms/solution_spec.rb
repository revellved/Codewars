describe 'Solution' do
  include Isograms

  it 'Fixed tests' do
    Test.assert_equals(is_isogram('Dermatoglyphics'), true)
    Test.assert_equals(is_isogram('isogram'), true)
    Test.assert_equals(is_isogram('aba'), false, 'same chars may not be adjacent')
    Test.assert_equals(is_isogram('moOse'), false, 'same chars may not be same case')
    Test.assert_equals(is_isogram('isIsogram'), false)
    Test.assert_equals(is_isogram(''), true, 'an empty string is a valid isogram')
  end
end
