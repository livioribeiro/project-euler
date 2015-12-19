UP_TO = 2000000

import math

def is_prime(num):
    if num <= 2:
        return True

    if num % 2 == 0:
        return False

    for i in range(3, math.ceil(math.sqrt(num)) + 1, 2):
        if num % i == 0:
            return False

    return True

if __name__ == '__main__':
    total = 2 + sum((i for i in range(3, UP_TO, 2) if is_prime(i)))
    print(total)
