# Problem 33: Digit cancelling fractions

## Problem statement

<p>The fraction <sup>49</sup>/<sub>98</sub> is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that <sup>49</sup>/<sub>98</sub> = <sup>4</sup>/<sub>8</sub>, which is correct, is obtained by cancelling the 9s.</p>
<p>We shall consider fractions like, <sup>30</sup>/<sub>50</sub> = <sup>3</sup>/<sub>5</sub>, to be trivial examples.</p>
<p>There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.</p>
<p>If the product of these four fractions is given in its lowest common terms, find the value of the denominator.</p>

## Comments

I suspect it'll be enough to see if (10a + b)/(10c + d) = b/c for all
a, b, c, d in 1..=9 such that b/c < 1.

...

Oops. Of course I meant (10a + b)/(10c + d) = a/d
