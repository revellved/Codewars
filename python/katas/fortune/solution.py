def fortune(f: int, p: int, c: int, n: int, i: int) -> bool:
    for _ in range(n-1):
        f = int(f * (100 + p) / 100 - c)
        c = int(c * (100 + i) / 100)
        if f < 0:
            return False
    return True

