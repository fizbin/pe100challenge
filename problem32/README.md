# Problem 32: Pandigital products

## Problem statement

<p>We shall say that an <var>n</var>-digit number is pandigital if it makes use of all the digits 1 to <var>n</var> exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.</p>
<p>The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.</p>
<p>Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.</p>
<p>HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.</p>

## Comments

This is the first problem that I'll be doing for the first time with
the Project Euler 100 Challenge! That is, with all the preceding
problems I solved them once ages ago (over a decade), when I first
discovered Project Euler. This is the first one I hadn't already
solved some other way.

...

Well that was a little fun. A bit of a slowdown from a minor mistake I
made w.r.t. the size of the product (it must be four digit) and some
minor fighting with the compiler over how to ask whether a `bool`
slice was all `true`, but it worked.
