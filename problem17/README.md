# Problem 17: Number letter counts

## Problem statement

If the numbers 1 to 5 are written out in words: one, two, three, four,
five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were
written out in words, how many letters would be used?


**NOTE**: Do not count spaces or hyphens. For example, 342 (three
hundred and forty-two) contains 23 letters and 115 (one hundred and
fifteen) contains 20 letters. The use of "and" when writing out
numbers is in compliance with British usage.

## Comments

This looks to be a bit annoying, as I can't rely on existing libraries
(that I know of) and the whole thing is special cases.

...

Well that turned out not to be so bad.

I actually found it faster to write a function that just does the
conversion to English and then count the letters than I did to write a
function that determined letter counts.
