"""
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
"""

import sys

# verify only multiples of the product of the prime numbers up to 20
COEF = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19

if __name__ == '__main__':
    for i in range(COEF, sys.maxsize, COEF):
        is_divisible = True

        # if n is divisible by 20, it is also divisible by its divisors, 10 and 5
        for d in range(11, 21):
            if i % d != 0:
                is_divisible = False
                break

        if is_divisible:
            print(i)
            break
