# Problem 26: Reciprocal cycles

## Problem Statement

A unit fraction contains 1 in the numerator. The decimal
representation of the unit fractions with denominators 2 to 10 are
given:

    1/2	= 	0.5
    1/3	= 	0.(3)
    1/4	= 	0.25
    1/5	= 	0.2
    1/6	= 	0.1(6)
    1/7	= 	0.(142857)
    1/8	= 	0.125
    1/9	= 	0.(1)
    1/10	= 	0.1

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It
can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest
recurring cycle in its decimal fraction part.

## Comments

If _1/n_ repeats after _d_ digits, then 10<sup><i>d</i></sup> is
equivalent to 1 modulo _n_. (just be sure to find the smallest such
_d_)

...

Well, that didn't work. It turns out that that statement is only true
for numbers where 1/n starts repeating immediately. Fortunately, a
related equation is true:

    (10^k) % n == (10^(k + d)) % n

for `k >= n`. No real rust issues this time around, just tying myself
into infinite loops.
