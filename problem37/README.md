# Problem 37: Truncatable primes

## Problem statement

<p>The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.</p>
<p>Find the sum of the only eleven primes that are both truncatable from left to right and right to left.</p>
<p class="info">NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.</p>

## Comments

I believe that the way to approach this is to find a `HashSet` of all
right-truncatable prime numbers and then find which ones are
left-truncatable.

This depends on the idea that the number of right-truncatable primes
is finite, whereas I'm not certain that the number of left-truncatable
primes is.

...

Well, I needed help from the rust discord channel to tell me about why
`HashSet` iterators are iterators of `&Item` instead of being
iterators of just `Item`. Fortunately, they also told me about
`.into_iter()`, which produces an iterator of the underlying value
type (and destroys the collection, but that's fine where I use it)
