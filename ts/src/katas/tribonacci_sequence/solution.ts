export function tribonacci(signature: number[], n: number): number[] {
    if (n === 0) return []; 
    const result: number[] = signature.slice(0, n);
    
    for (let i = 3; i < n; i++)
        result[i] = result[i - 1] + result[i - 2] + result[i - 3];

    return result;
}
