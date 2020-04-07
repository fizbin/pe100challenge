# Problem 38: Pandigital multiples

## Problem statement

<p>Take the number 192 and multiply it by each of 1, 2, and 3:</p>
<blockquote>192 × 1 = 192<br>
192 × 2 = 384<br>
192 × 3 = 576</blockquote>
<p>By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)</p>
<p>The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).</p>
<p>What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , <var>n</var>) where <var>n</var> &gt; 1?</p>

## Comments

Well obviously the answer will start with `9`. I'm tempted to try do
this by hand since I think it's pretty straightforward to get to the
answer. However, I'll do it in rust just the same.

...

Yeah, a bit long in the code, but pretty straightforward. It'd
probable be easier if I made use of string/integer conversions, but
since I didn't need them, I didn't.

I'm still stubbing my toe on all the different integer types. I wonder
how real `rust` programmers ever get anything done getting all their
integer types in a row.
