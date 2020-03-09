# Problem 20: Factorial digit sum

## Problem statement

`n!` means `n × (n − 1) × ... × 3 × 2 × 1`

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

## Comments

This looks like a job for `rug`, especially since I already fought
with the borrow checker to work out how one does a digit sum in
[problem 16](../problem16/).

...

Yep, pretty easy. Go `rug`.
