# frozen_string_literal: true

describe 'Solution' do
  it 'Fixed tests' do
    extend ValidBraces

    Test.assert_equals(valid_braces('()'), true)
    Test.assert_equals(valid_braces('[(])'), false)
  end
end
