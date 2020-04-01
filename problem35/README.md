# Problem 35: Circular primes

## Problem statement

<p>The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.</p>
<p>There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.</p>
<p>How many circular primes are there below one million?</p>

## Comments

I guess make a sieve to find the primes, and then... I guess rotate
through everything? Not sure what the best way to do that is.

...

Man, I almost thought I had my first rust program that would compile
the first time. Sadly, no, I forgot all the `&` when calling
`check`. Maybe next time.
