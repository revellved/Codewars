# frozen_string_literal: true

describe 'Valid Braces spec solution' do
  include ValidBraces

  it 'Fixed tests' do
    Test.assert_equals(valid_braces('()'), true)
    Test.assert_equals(valid_braces('[(])'), false)
  end
end
