def is_prime(num):
    if num <= 2:
        return True

    if num % 2 == 0:
        return False

    for i in range(3, math.ceil(math.sqrt(num)) + 1, 2):
        if num % i == 0:
            return False

    return True

for i in range(6, 300, 6):
    p1 = '{:7}'.format(i - 1) if is_prime(i - 1) else (' ' * 7)
    p1 = '{:7}'.format(i + 1) if is_prime(i + 1) else (' ' * 7)
    print('{:7}{:7}{:7}'.format(i - 1, i, i + 1))
