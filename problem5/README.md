# Problem 5

## Problem statement

2520 is the smallest number that can be divided by each of the numbers
from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all
of the numbers from 1 to 20?

## Comments

This seems like a job for a function that finds the least common
multiple of two values and then using the `.fold` method on
ranges.

...

And indeed it was, modulo a few quirks of rust that I'm still learning
(like forgetting to specify function return values each time).


## Source

[Source code](https://github.com/fizbin/pe100challenge/blob/master{{page.url}}src/main.rs)
