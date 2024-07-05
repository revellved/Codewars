# frozen_string_literal: true

describe 'Build Tower spec solution' do
  include BuildTower

  it 'Fixed test' do
    Test.assert_equals(tower_builder(1), ['*'])
    Test.assert_equals(tower_builder(2), [' * ', '***'])
    Test.assert_equals(tower_builder(3), ['  *  ', ' *** ', '*****'])
  end
end
