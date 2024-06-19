# frozen_string_literal: true

describe 'Roman Numerals Encoder' do
  it 'Fixed tests' do
    extend RomanNumeralsEncoder

    expect(solution(1)).to eq('I'), "solution(1),'I'"
    expect(solution(4)).to eq('IV'), "solution(4),'IV'"
    expect(solution(6)).to eq('VI'), "solution(6),'VI'"
    expect(solution(14)).to eq('XIV'), "solution(14),'XIV"
    expect(solution(21)).to eq('XXI'), "solution(21),'XXI'"
    expect(solution(89)).to eq('LXXXIX'), "solution(89),'LXXXIX'"
    expect(solution(91)).to eq('XCI'), "solution(91),'XCI'"
    expect(solution(984)).to eq('CMLXXXIV'), "solution(984),'CMLXXXIV'"
    expect(solution(1000)).to eq('M'), 'solution(1000),(M'
    expect(solution(1889)).to eq('MDCCCLXXXIX'), "solution(1889),'MDCCCLXXXIX'"
    expect(solution(1989)).to eq('MCMLXXXIX'), "solution(1989),'MCMLXXXIX'"
  end
end
