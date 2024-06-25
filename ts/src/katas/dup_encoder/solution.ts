export function duplicateEncode(word: string): string {
  const charCount: { [key: string]: number } = {};
  const lowerWord = word.toLowerCase();
  let result = "";

  for (let char of lowerWord) {
    charCount[char] = (charCount[char] || 0) + 1;
  }

  for (let char of lowerWord) {
    result += charCount[char] > 1 ? ")" : "(";
  }

  return result;
}
