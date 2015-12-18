"""
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""

from itertools import permutations

if __name__ == '__main__':
    break_it = False
    for n in range(1, 1000):
        for m in range(n + 1, 1000):
            # https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
            # Euclid's formula
            b, a, c = m**2 - n**2, 2 * m * n, m**2 + n**2

            if a**2 + b**2 == c**2 and a + b + c == 1000:
                print('{}^2 + {}^2 == {}^2 == {} + {} == {}'.format(a, b, c, a**2, b**2, c**2))
                print('{} + {} + {} == 1000'.format(a, b, c))
                print('{} * {} * {} == {}'.format(a, b, c, a * b * c))

                break_it = True
                break

        if break_it: break
