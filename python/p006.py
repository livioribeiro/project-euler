"""
The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the
square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and
the square of the sum.
"""

def sum_square_difference(up_to):
    return int((sum(range(up_to + 1)) ** 2) - sum_of_squares(up_to))

# http://mathschallenge.net/library/number/sum_of_squares
# "The sum of the first n squares, 12+22+...+n2 = n(n+1)(2n+1)/6."
def sum_of_squares(up_to):
    return up_to * (up_to + 1) * (2 * up_to + 1) / 6

if __name__ == '__main__':
    print(sum_square_difference(100))
