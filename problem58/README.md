# Problem 58: Spiral primes

## Problem Statement

<p>Starting with 1 and spiralling anticlockwise in the following way, a square spiral with side length 7 is formed.</p>
<p class="center monospace"><span class="red"><b>37</b></span> 36 35 34 33 32 <span class="red"><b>31</b></span><br />
38 <span class="red"><b>17</b></span> 16 15 14 <span class="red"><b>13</b></span> 30<br />
39 18 <span class="red"> <b>5</b></span>  4 <span class="red"> <b>3</b></span> 12 29<br />
40 19  6  1  2 11 28<br />
41 20 <span class="red"> <b>7</b></span>  8  9 10 27<br />
42 21 22 23 24 25 26<br /><span class="red"><b>43</b></span> 44 45 46 47 48 49</p>
<p>It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.</p>
<p>If one complete new layer is wrapped around the spiral above, a square spiral with side length 9 will be formed. If this process is continued, what is the side length of the square spiral for which the ratio of primes along both diagonals first falls below 10%?</p>

## Comments

The numbers along the diagonals are given by the formulas:

4n<sup>2</sup> - 2n + 1

4n<sup>2</sup> + 1

4n<sup>2</sup> + 2n + 1

4n<sup>2</sup> + 4n + 1

So I guess what I'll do is make the sieve of prime numbers up to 10K
and then go around the squares counting up primes and totals until the
number of primes is less than one-tenth the total number of numbers.

...

Okay, so this requires going much further than any sieve will allow.
Therefore, what I'm going to do is make a sieve up to 1 million and then
convert that into a list of primes and use that to check primality up to 10<sup>12</sup>

...

Turns out I only needed a sieve up to 100,000 to finish this; the final square
contained numbers upwards of 688 million.
