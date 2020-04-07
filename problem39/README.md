# Problem 39: Integer right triangles

## Problem statement

<p>If <i>p</i> is the perimeter of a right angle triangle with integral length sides, {<i>a</i>,<i>b</i>,<i>c</i>}, there are exactly three solutions for <i>p</i> = 120.</p>
<p>{20,48,52}, {24,45,51}, {30,40,50}</p>
<p>For which value of <i>p</i> ≤ 1000, is the number of solutions maximised?</p>

## Comments

I think I'll make an array up to 1000 to keep a count of how many
solutions have each perimeter, and iterate through right triangles
using the formula shown [here on
wikipedia](https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple) -

      a=k*(m^2 - n^2)     b=k*(2mn)    c=k*(m^2+n^2)

just need to iterate through `m > n` and `m` and `n` relatively prime.

...

