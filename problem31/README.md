# Problem 31: Coin sums

## Problem Statement

<p>In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:</p>
<blockquote>1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).</blockquote>
<p>It is possible to make £2 in the following way:</p>
<blockquote>1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p</blockquote>
<p>How many different ways can £2 be made using any number of coins?</p>

## Comments

This looks like a job for dynamic programming.

...

A bit trickier than initially imagined because initially I was working
out the wrong thing and counting each permutation of coins
separately. A little bit surprised that I need to specify the `u64`
type on `sum()` at the end, but not elsewhere. I don't understand why
the compiler can't infer that.
