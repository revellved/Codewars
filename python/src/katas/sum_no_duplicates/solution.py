from functools import reduce

def sum_no_duplicates(l: list[int]) -> int:
    return reduce(lambda acc, el: acc + el if l.count(el) == 1 else acc, l, 0)
