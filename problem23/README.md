# Problem 23: Non-abundant sums

## Problem Statement

A perfect number is a number for which the sum of its proper divisors
is exactly equal to the number. For example, the sum of the proper
divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28
is a perfect number.

A number _n_ is called deficient if the sum of its proper divisors is
less than _n_ and it is called abundant if this sum exceeds _n_.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the
smallest number that can be written as the sum of two abundant numbers
is 24. By mathematical analysis, it can be shown that all integers
greater than 28123 can be written as the sum of two abundant
numbers. However, this upper limit cannot be reduced any further by
analysis even though it is known that the greatest number that cannot
be expressed as the sum of two abundant numbers is less than this
limit.

Find the sum of all the positive integers which cannot be written as
the sum of two abundant numbers.

## Comments

I suspect that the thing to do here is build a vector of all abundant
numbers up to 28123, convert that into a bitvec, and then repeatedly
shift that bitvec, bitwise or-ing it into a larger bitvec to get a
bitvec of all the possible sums, and then iterate over that to find
the sum.

..

Well, that was a bear. The fact that Rust has two different crates
called `bitvec` and `bit-vec` that both supply a type called `BitVec`
with very different functionality is not helpful, but also not good is
an error in my initial implementation of `is_abundant` that allowed
`0` as an abundant number.

I did eventually come up with a fairly fast program to do this, but
needed help from the rust IRC channel.

