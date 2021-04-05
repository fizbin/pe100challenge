# Problem 49: Prime permutations

## Problem statement

The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.

There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

What 12-digit number do you form by concatenating the three terms in this sequence?

## Comments

So, we make a prime sieve ala problem 35 and then I guess search it for pairs of primes that are permutations of each other
(need to come up with a good way to check that) that have an arithmetic mean that's also a prime and also a permutation.

Two numbers that are permutations of each other will have the same residue modulo nine; maybe there's something that can
be done with that.

...

I abandoned the initial approach of taking a number and making an iterator that would give me each permutation which I'd
then check, etc. Instead, I dropped each four-digit prime into a `HashMap<[u8; 4], u32>` that mapped the sorted digits
to the prime. The tricky part then was writing a function that would give me an iterator over combinations of 3 items
at a time from a `Vec`. This was full of rust generics fun, including learning some new things about lifetime
parameters and how to specify associated types when returning a value with associated types.