# Problem 34: Digit factorials

## Problem Statement

<p>145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.</p>
<p>Find the sum of all numbers which are equal to the sum of the factorial of their digits.</p>
<p>Note: as 1! = 1 and 2! = 2 are not sums they are not included.</p>

## Comments

Should be straightforward, though I probably want to precompute the
factorial and have that be just an array lookup.

...

Okay, that was *NOT* straightforward, because rust has a view of
`static` that I wasn't familiar with at all. It was fine once I
found the `lazy_static` crate, but I don't like how much unfamiliar
boilerplate there is to using that.
