from typing import List

def two_sum(nums: List[int], t: int) -> tuple[int, int]:
    for i, x in enumerate(nums):
        for j, y in enumerate(nums):
            if i != j and x + y == t:
                return (i, j)
    return (0,0) 
