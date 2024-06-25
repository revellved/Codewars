def is_square(num: int):
    if num < 0:
        return False
    sqrt_num = int(num ** 0.5)
    return sqrt_num * sqrt_num == num
