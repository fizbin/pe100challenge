# Problem 9

## Problem statement

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

    a^2 + b^2 = c^2

For example, `3^2 + 4^2 = 9 + 16 = 25 = 5^2`.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

## Comments

I've taken a bit long thinking about this, because my schedule
wouldn't let me get back to PE100 stuff until today. As a consequence,
I'm going to iterate through Pythagorean triplets in what might be a
very non-intuitive way by using [Euclid's
Formula](https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple)

...

Yep, straightforward. I do wonder if there's a more idiomatic way to
break out of a doubly-nested loop in rust than to push both loops off
into a separate function that you then return out of.


## Source

[Source code](https://github.com/fizbin/pe100challenge/blob/master{{page.url}}src/main.rs)
