# Problem 51: Prime digit replacements

## Problem statement

<p>By replacing the 1<sup>st</sup> digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.</p>
<p>By replacing the 3<sup>rd</sup> and 4<sup>th</sup> digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.</p>
<p>Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.</p>

## Comments

My plan was to insert each prime under 1000000 into all the different possible "mask families" that
prime belonged in. The hard part was making an iterator that could handle all the possible masks
that you need to deal with when you have a prime with multiple digits the same. This taught me a bit
about working with `String` and manipulating `String` and `&str` values to construct each mask.