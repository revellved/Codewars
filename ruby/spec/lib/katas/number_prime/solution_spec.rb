# frozen_string_literal: true

describe 'prime?' do
  it 'Should return false for non-prime numbers.' do
    extend NumberPrime

    Test.assert_equals(prime?(4), false)
    Test.assert_equals(prime?(100), false)
    Test.assert_equals(prime?(999), false)
    Test.assert_equals(prime?(958_297), false)
    Test.assert_equals(prime?(-7), false)
  end

  it 'Should return true for prime numbers.' do
    extend NumberPrime

    Test.assert_equals(prime?(2), true)
    Test.assert_equals(prime?(3), true)
    Test.assert_equals(prime?(5), true)
    Test.assert_equals(prime?(457), true)
    Test.assert_equals(prime?(39_229), true)
  end
end
