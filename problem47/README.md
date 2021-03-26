# Problem 47: Distinct Primes factors

## Problem statement

<p>The first two consecutive numbers to have two distinct prime factors are:</p>
<p class="margin_left">14 = 2 × 7<br />15 = 3 × 5</p>
<p>The first three consecutive numbers to have three distinct prime factors are:</p>
<p class="margin_left">644 = 2² × 7 × 23<br />645 = 3 × 5 × 43<br />646 = 2 × 17 × 19.</p>
<p>Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?</p>

## Comments

I should be able to take existing prime seive code (like that used
for problem 35) and modify it to add 1 to a "distinct prime factors"
count instead of setting a boolean to false.

...

yes, it was that straightforward.