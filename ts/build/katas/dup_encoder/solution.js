export function duplicateEncode(word) {
    const charCount = {};
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
//# sourceMappingURL=solution.js.map