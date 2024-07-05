# frozen_string_literal: true

describe 'Basic tests' do
  include BuildTower

  it 'should pass basic tests' do
    Test.assert_equals(tower_builder(1), ['*'])
    Test.assert_equals(tower_builder(2), [' * ', '***'])
    Test.assert_equals(tower_builder(3), ['  *  ', ' *** ', '*****'])
  end
end
