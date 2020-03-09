# Problem 21: Amicable numbers

## Problem statement

Let _d(n)_ be defined as the sum of proper divisors of _n_ (numbers less
than _n_ which divide evenly into _n_).

If _d(a)_ = _b_ and _d(b)_ = _a_, where _a_ ≠ _b_, then _a_ and _b_
are an amicable pair and each of _a_ and _b_ are called amicable
numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20,
22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284
are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

## Comments

Determining the proper divisors of _x_ could be done with a sieve-like
structure like I used for [problem 12](../problem12/), but given that
I can stop iterating when the divisor is less than the square root of
the number I'm checking, it won't be so bad to just work out the sum
by testing everything.

...

Okay, a bit of fighting with the compiler because of method names
that are diffetent between Rust and Python. (Specifically, `append` in
python means the same as `push` does, whereas `append` in Rust is like
python's `extend`).
