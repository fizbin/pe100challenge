# Problem 57: Square root convergents

## Problem statement

<p>It is possible to show that the square root of two can be expressed as an infinite continued fraction.</p>
<p class="center">$\sqrt 2 =1+ \frac 1 {2+ \frac 1 {2 +\frac 1 {2+ \dots}}}$</p>
<p>By expanding this for the first four iterations, we get:</p>
<p>$1 + \frac 1 2 = \frac  32 = 1.5$<br />
$1 + \frac 1 {2 + \frac 1 2} = \frac 7 5 = 1.4$<br />
$1 + \frac 1 {2 + \frac 1 {2+\frac 1 2}} = \frac {17}{12} = 1.41666 \dots$<br />
$1 + \frac 1 {2 + \frac 1 {2+\frac 1 {2+\frac 1 2}}} = \frac {41}{29} = 1.41379 \dots$<br /></p>
<p>The next three expansions are $\frac {99}{70}$, $\frac {239}{169}$, and $\frac {577}{408}$, but the eighth expansion, $\frac {1393}{985}$, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.</p>
<p>In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?</p>

## Comments

So we basically have a recurrence where (u<sub>1</sub>, v<sub>1</sub>) = (3, 2) and

(u<sub>n</sub>, v<sub>n</sub>) = (2 * v<sub>n-1</sub> + u<sub>n-1</sub>, v<sub>n-1</sub> + u<sub>n-1</sub>)

And we need to count how many times we have u with more digits than v.

...

Yep, that was it. A bit tricky to remember to sprinkle `&` in places to keep the borrow checker happy, but fine other than that.
