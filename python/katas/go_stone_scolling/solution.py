def determine_winner(board):
    black_count = sum(row.count("B") for row in board)
    white_count = sum(row.count("W") for row in board)

    if black_count > white_count:
        return ("B", black_count)
    elif white_count > black_count:
        return ("W", white_count)
    else:
        return ("T", black_count)
