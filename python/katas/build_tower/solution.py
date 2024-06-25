from typing import List

def tower_builder(n_floors: int) -> List[str]:
    tower = []
    width = n_floors * 2 - 1
    for i in range(1, n_floors + 1):
        stars = '*' * (2*i - 1)
        spaces = ' ' * ((width - len(stars)) // 2)
        tower.append(spaces + stars + spaces)
    return tower

