# frozen_string_literal: true

describe 'Basic tests' do
  include CommonDeniminators

  it 'should pass basic tests' do
    expect(convert_fracts([[1, 2], [1, 3], [1, 4]])).to eq([[6, 12], [4, 12], [3, 12]])
  end
end
