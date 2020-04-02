from __future__ import division
from __future__ import print_function


def main():
    mysum = [0]

    def print_n_add(base_2_palindrome):
        if check_base10_palindrome(base_2_palindrome):
            print(base_2_palindrome)
            mysum[0] += base_2_palindrome

    find_all_palindromes(print_n_add)
    print('sum is', mysum[0])


def find_all_palindromes(found_func):
    found_func(1)

    def find_with_inner_digits(inner_digit_count):
        high_added = 1 << (inner_digit_count + 1)
        def inner_func(palindrome):
            found_func(high_added + 2*palindrome + 1)
        palindrome_loop(n_inner_digits, inner_func)

    for n_inner_digits in range(19):
        find_with_inner_digits(n_inner_digits)


def palindrome_loop(n_digits, found_func):
    if n_digits == 0:
        found_func(0)
    elif n_digits == 1:
        found_func(0)
        found_func(1)
    else:
        def zero_func(palindrome):
            found_func(2*palindrome)
        palindrome_loop(n_digits - 2, zero_func)
        high_added = 1 << (n_digits - 1)

        def one_func(palindrome):
            found_func(high_added + 2*palindrome + 1)
        palindrome_loop(n_digits - 2, one_func)


def check_base10_palindrome(n):
    if n >= 1000000:
        return False
    if n < 10:
        return True
    pow_10 = 10
    ndig = 1
    while pow_10 < n:
        pow_10 *= 10
        ndig += 1
    return check_base10_palindrome_2(n, ndig)


def check_base10_palindrome_2(n, ndigits):
    if ndigits < 2:
        return True
    top_of_n = n
    pow_10 = 1
    for _ in range(ndigits-1):
        top_of_n //= 10
        pow_10 *= 10
    return ((top_of_n == n % 10)
            and check_base10_palindrome_2((n % pow_10) // 10, ndigits - 2))


if __name__ == '__main__':
    main()
