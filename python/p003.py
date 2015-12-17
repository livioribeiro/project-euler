"""
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
"""

import math

def is_prime(num):
    if num <= 2:
        return True

    if num % 2 == 0:
        return False

    for i in range(3, math.ceil(math.sqrt(num)), 2):
        if num % i == 0:
            return False

    return True

INPUT = 600851475143

if __name__ == '__main__':
    for i in range(math.ceil(math.sqrt(INPUT)), 1, -2):
        if INPUT % i == 0 and is_prime(i):
            print(i)
            break
