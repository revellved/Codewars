pub fn spiralize(size: usize) -> Vec<Vec<u8>> {
    let mut spiral = vec![vec![0; size]; size];
    let (mut x, mut y, mut dx, mut dy) = (0, 0, 1, 0);

    while x < size && y < size && spiral[y][x] == 0 {
        spiral[y][x] = 1;

        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0
            || nx >= size as i32
            || ny < 0
            || ny >= size as i32
            || spiral[ny as usize][nx as usize] == 1
        {
            let (new_dx, new_dy) = (-dy, dx);
            if (x + new_dx as usize) < size
                && (y + new_dy as usize) < size
                && spiral[y + new_dy as usize][x + new_dx as usize] == 0
            {
                dx = new_dx;
                dy = new_dy;
            }
        }

        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
