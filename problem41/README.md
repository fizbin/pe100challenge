# Problem 41: Pandigital prime

## Problem statement

<p>We shall say that an <i>n</i>-digit number is pandigital if it makes use of all the digits 1 to <i>n</i> exactly once. For example, 2143 is a 4-digit pandigital and is also prime.</p>
<p>What is the largest <i>n</i>-digit pandigital prime that exists?</p>

## Comments

Well, it obviously can't have eight or nine digits, because the sum of
the numbers 1 to 8 are 36, and 1 to 9 are 45. Therefore, any 1 to 8
pandigital number or any 1 to 9 pandigital number will be divisible by
three. (and therefore not prime) Similarly for any 5- or 6-digit
pandigital. Since we know that there are four-digit pandigital primes,
it remains only to check for a 7-digit pandigital prime, and then a
four-digit one.

..

Okay, I took two submissions on this because the first time I was
iterating the permutations in an incorrect order so I found a 7-digit
pandigital prime, but the first one I found wasn't the largest. Once I
fixed my iteration, it was good.

This iteration technique might be worth paying attention to. It
basically runs a point up and down the number trying to change the
value at that index. Whenever it gets a value in any spot not the
last, it moves the pointer further down the list of digits - setting a
value in the last spot means we have a complete number and should
evaluate. Whenever it discovers that it's used all the possible
values, it sets the current spot to an out-of-range value and moves
the pointer back toward the beginning of the list of digits. The
out-of-range value is chosen so that when the pointer moves back down
it'll start back at the top of the range of valid values.

Anyway, this is a basic technique for many counting-esque tasks:
change the least significant position that you can, and every time you
change position `<n>` make sure that positions `<n+1>`...`<end>` are
set to the limit you want (max val or min val), depending on the order
you're iterating in.
