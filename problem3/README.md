# Problem 3

## Problem statement

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

## Comments

I just wrote a quick-and-dirty search for the smallest prime factor of
a number that tries 2 and 3 and then searches by trying one less than
a multiple of 6 and one more than a multiple of 6.

I then repeatedly divide my goal by its smallest prime factor until I
end up with a prime number and that's the answer.


## Source

[Source code](https://github.com/fizbin/pe100challenge/blob/master{{page.url}}src/main.rs)
