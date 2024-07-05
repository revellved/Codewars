# frozen_string_literal: true

describe 'Common Denominators spec solution' do
  include CommonDenominators

  it 'Fixed tests' do
    expect(convert_fracts([[1, 2], [1, 3], [1, 4]])).to eq([[6, 12], [4, 12], [3, 12]])
  end
end
