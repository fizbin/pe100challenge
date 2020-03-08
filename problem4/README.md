# Problem 4

## Problem statement

A palindromic number reads the same both ways. The largest palindrome
made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit
numbers.

## Comments

This is going to be the first problem that involves string
manipulation in rust. Fortunately, I can guarantee that I'm only using
ASCII, so I can just use `.as_bytes` on a string to get the bytes.

I iterated the way I did so that the first palindrome I found would
likely be the one I'd want: given that two numbers sum to some value
`factorsum`, the largest product will be the one with the smallest
difference between the two factors. I then start with the largest
possible sum and work my way backwards. The instant I find a
palindrome, I cut all `factorsum` loops off once the product drops
below that value.


## Source

[Source code](https://github.com/fizbin/pe100challenge/blob/master{{page.url}}src/main.rs)
