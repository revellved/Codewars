# frozen_string_literal: true

describe 'Move Zeros spec solution' do
  include MoveZeros

  it 'Fixed tests' do
    expect(moveZeros([1, 2, 0, 1, 0, 1, 0, 3, 0, 1])).to eq([1, 2, 1, 1, 3, 1, 0, 0, 0, 0])
  end
end
