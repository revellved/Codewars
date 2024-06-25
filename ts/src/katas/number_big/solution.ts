export function narcissistic(value: number): boolean {
  let nums: number[] = Array.from(String(value), Number);
  let len: number = nums.length;

  return nums.reduce((num1: number, num2: number) => {
    return Math.pow(num1, len) + Math.pow(num2, len);
  }) == value;
}

export function narcissistic_js(value: number) {
  let nums = Array.from(String(value), Number);
  let len = nums.length;

  return nums.reduce((num1, num2) => {
    return Math.pow(num1, len) + Math.pow(num2, len);
  }, 0) == value;
}
