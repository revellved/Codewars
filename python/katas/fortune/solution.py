type Num = int|float

def fortune(f: Num, p: Num, c: Num, n: Num, i: Num) -> bool:
    for _ in range(int(n)-1):
        f = int(f * (100 + p) / 100 - c)
        c = int(c * (100 + i) / 100)
        if f < 0:
            return False
    return True

