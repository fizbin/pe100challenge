# Problem 40: Champernowne's constant

## Problem statement

<p>An irrational decimal fraction is created by concatenating the positive integers:</p>
<p style="text-align:center;">0.12345678910<span style="color:#dd0000;font-size:14pt;">1</span>112131415161718192021...</p>
<p>It can be seen that the 12<sup>th</sup> digit of the fractional part is 1.</p>
<p>If <i>d</i><sub><i>n</i></sub> represents the <i>n</i><sup>th</sup> digit of the fractional part, find the value of the following expression.</p>
<p style="text-align:center;"><i>d</i><sub>1</sub> × <i>d</i><sub>10</sub> × <i>d</i><sub>100</sub> × <i>d</i><sub>1000</sub> × <i>d</i><sub>10000</sub> × <i>d</i><sub>100000</sub> × <i>d</i><sub>1000000</sub></p>

## Comments

It's late and I don't feel like I can be clever, so I'm just going to
brute force it with `.to_string()`.

...


Yeah, it turns out when you aren't trying to be clever you don't have
to write nearly as much code.
