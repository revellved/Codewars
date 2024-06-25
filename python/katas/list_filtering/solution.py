from typing import List

def filter_list(l: List[int|str]) -> List[int|str]:
    return list(filter(lambda el: type(el) == int, l))

