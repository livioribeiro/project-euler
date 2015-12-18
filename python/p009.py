"""
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""

from itertools import permutations

if __name__ == '__main__':
    # # Slower but interesting solution
    # for (a, b, c) in permutations(range(1, 500), 3):
    #     if a**2 + b**2 == c**2 and a + b + c == 1000:
    #         print('{}^2 + {}^2 == {}^2 == {} + {} == {}'.format(a, b, c, a**2, b**2, c**2))
    #         print('{} + {} + {} == 1000'.format(a, b, c))
    #         print('{} * {} * {} == {}'.format(a, b, c, a * b * c))
    #
    #         break

    break_it = False
    for a in range(1, 500):
        for b in range(a + 1, 500):
            for c in range(b + 1, 500):
                if a**2 + b**2 == c**2 and a + b + c == 1000:
                    print('{}^2 + {}^2 == {}^2 == {} + {} == {}'.format(a, b, c, a**2, b**2, c**2))
                    print('{} + {} + {} == 1000'.format(a, b, c))
                    print('{} * {} * {} == {}'.format(a, b, c, a * b * c))

                    break_it = True
                    break
            if break_it: break
        if break_it: break
