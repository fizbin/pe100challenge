# Problem 16: Power digit sum

## Problem statement

2<sup>15</sup> = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2<sup>1000</sup>?

## Comments

This looks like a job for `rug`. There are probably more clever ways,
but that seems like the right tool for the job.

...

So that took way too long, and it was all me fighting with the borrow
checker.

Though, to be fair, given the way the borrow checker exists the type
of the method `rug::Integer::div_rem` makes no sense to me - I can't
imagine a situation where you'd want to call that instead of
`rug::Integer::div_rem_ref` - the latter has exactly the semantics of
the former, except that it doesn't take ownership of the divisor. Why
would you want to pass ownership of the divisor away? That can't do
anything.

Example fighting with the borrow checker:

    error[E0382]: use of moved value: `ten`
      --> src/main.rs:11:43
       |
    9  |     let ten = Integer::from(10);
       |         --- move occurs because `ten` has type `rug::integer::big::Integer`, which does not implement the `Copy` trait
    10 |     while longpow.cmp0() == Ordering::Greater {
    11 |         let (quot, rem) = longpow.div_rem(ten);
       |                                           ^^^ value moved here, in previous iteration of loop
