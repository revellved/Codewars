from typing import List

def tower_builder(n: int) -> List[str]:
    return [("*" * (i*2-1)).center(n*2-1) for i in range(1, n+1)]
