def dont_give_me_five(start: int, end: int) -> int:
    return sum('5' not in str(i) for i in range(start, end + 1))
