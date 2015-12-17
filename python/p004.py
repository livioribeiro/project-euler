"""
A palindromic number reads the same both ways. The largest palindrome made from the product of
two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
"""

import itertools

def get_level(num):
    level = 1
    divisor = 10

    while num / divisor >= 1:
        level += 1
        divisor *= 10

    return level

def get_digit_at(pos, num):
    return int((num % (10 ** pos)) / (10 ** (pos - 1)))

def is_palindrome(num):
    i = get_level(num)
    j = 1

    while i > j:
        if get_digit_at(i, num) != get_digit_at(j, num):
            return False

        i -= 1
        j += 1

    return True

if __name__ == '__main__':
    largest = 0
    largest_factor1 = 0
    largest_factor2 = 0

    for i, j in itertools.product(range(999, 100, -1), repeat=2):
        num = i * j

        if i % 10 == 0 or j % 10 == 0 or num % 10 == 0:
            continue

        if num > largest and is_palindrome(num):
            largest = num
            largest_factor1 = i
            largest_factor2 = j

    print("{} = {} * {}".format(largest, largest_factor1, largest_factor2))
