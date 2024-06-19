# frozen_string_literal: true

describe 'weirdcase' do
  it 'should return the correct value for a single word' do
    extend WeirdStringCase

    Test.assert_equals(weirdcase('This'), 'ThIs')
    Test.assert_equals(weirdcase('is'), 'Is')
  end

  it 'should return the correct value for multiple words' do
    extend WeirdStringCase

    Test.assert_equals(weirdcase('This is a test'), 'ThIs Is A TeSt')
  end
end
