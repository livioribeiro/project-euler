"""
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
"""

import functools

FACTORS = [0, 3, 5, 6, 9, 10, 12]

if __name__ == "__main__":
    total = 0

    # 990 is the last multiple of 15 below 1000
    for i in range(15, 1000, 15):
        total += functools.reduce(lambda acc, elem: acc + i - elem, FACTORS, 0)

    # multiples of 3 and 5 remaining
    total += 993 + 995 + 996 + 999

    print(total)
