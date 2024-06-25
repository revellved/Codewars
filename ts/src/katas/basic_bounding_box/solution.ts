// Return an array with the same shape as the input
export function boundingBox(imageArray: number[][]): number[][] {
    const rows = imageArray.length;
    const cols = imageArray[0]?.length || 0;

    if (rows === 0 || cols == 0) return []

    let top = rows;
    let bottom = 0;
    let left = cols;
    let right = 0;

    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (imageArray[i][j] === 1) {
                top = Math.min(top, i);
                bottom = Math.max(bottom, i);
                left = Math.min(left, j);
                right = Math.max(right, j);
            }
        }
    }

    const boundingBox: number[][] = Array.from({ length: rows }, () => Array(cols).fill(0));

    for (let i = top; i <= bottom; i++) {
        boundingBox[i][left] = 1;
        boundingBox[i][right] = 1;
    }

    for (let j = left; j <= right; j++) {
        boundingBox[top][j] = 1;
        boundingBox[bottom][j] = 1;
    }

    return boundingBox;
}

