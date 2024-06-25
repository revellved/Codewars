export function tribonacci(signature, n) {
    if (n === 0)
        return [];
    const result = signature.slice(0, n);
    for (let i = 3; i < n; i++)
        result[i] = result[i - 1] + result[i - 2] + result[i - 3];
    return result;
}
//# sourceMappingURL=solution.js.map