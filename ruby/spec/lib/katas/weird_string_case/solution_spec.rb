# frozen_string_literal: true

describe 'Weirdcase spec solution' do
  include WeirdStringCase

  context 'Fixed tests' do
    it 'should return the correct value for a single word' do
      Test.assert_equals(weirdcase('This'), 'ThIs')
      Test.assert_equals(weirdcase('is'), 'Is')
    end

    it 'should return the correct value for multiple words' do
      Test.assert_equals(weirdcase('This is a test'), 'ThIs Is A TeSt')
    end
  end
end
