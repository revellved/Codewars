from typing import List


def is_valid_walk(walk: List[str]):
    if len(walk) != 10:
        return False

    count = {'n': 0, 's': 0, 'e': 0, 'w': 0}

    for direction in walk:
        count[direction] += 1

    return count['n'] == count['s'] and count['e'] == count['w']
