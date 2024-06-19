# frozen_string_literal: true

describe 'Delete occurrences of an element if it occurs more than n times' do
  it 'Basic tests' do
    extend DeleteNth

    expect(delete_nth([20, 37, 20, 21], 1)).to eq([20, 37, 21]), 'max_e = 1'
    expect(delete_nth([1, 1, 3, 3, 7, 2, 2, 2, 2], 3)).to eq([1, 1, 3, 3, 7, 2, 2, 2]), 'max_e =3'
    expect(delete_nth([12, 39, 19, 39, 39, 19, 12], 1)).to eq([12, 39, 19]), 'max_e = 1'
  end
end
