export function narcissistic(value) {
    let nums = Array.from(String(value), Number);
    let len = nums.length;
    return nums.reduce((sum, num) => {
        return sum + Math.pow(num, len);
    }, 0) == value;
}
//# sourceMappingURL=solution.js.map