# Problem 24: Lexicographic permutations

## Problem Statement

A permutation is an ordered arrangement of objects. For example, 3124
is one possible permutation of the digits 1, 2, 3 and 4. If all of the
permutations are listed numerically or alphabetically, we call it
lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

    012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2,
3, 4, 5, 6, 7, 8 and 9?

## Comments

This one seems pretty straightforward: if I'm looking for the _n_th
permutation of _m_ items, it's going to begin with the smallest of
those _m_ items if `n <= (m-1)!`, with the second smallest if
`(m-1)! < n <= 2*(m-1)!`, etc.

So this should be easy and fast, just watch closely for off-by-one
errors.

...

Yes, as straightforward as I thought it'd be. Some fighting with
integer types and entirely too much fighting with types initially when
I used a `Vec<u8>` instead of a `Vec<&str>`, but switching types made
things much nicer.

I do wonder why Rust doesn't provide a `.join` method on iterators of
`&str`, forcing one to use `collect` first.

