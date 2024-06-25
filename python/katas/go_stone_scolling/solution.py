from typing import List


def determine_winner(board: List[str]) -> tuple[str, int]:
    black_count: int = sum(row.count("B") for row in board)
    white_count: int = sum(row.count("W") for row in board)

    if black_count > white_count:
        return ("B", black_count)
    elif white_count > black_count:
        return ("W", white_count)
    else:
        return ("T", black_count)
