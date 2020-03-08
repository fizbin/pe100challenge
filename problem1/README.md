# Problem 1

## Problem statement

If we list all the natural numbers below 10 that are multiples of 3 or
5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

## Comments

Straightforward, just count and add to the sum if the number is a
multiple of 3 or 5, which is checked just as in C.

I suppose this one could be done by hand using the formula for
\sum_{i=1}^k i, but straight-forward brute force was very sikmple and
fast.

## Source

[Source code](https://github.com/fizbin/pe100challenge/blob/master{{page.url}}/src/main.rs)
