# Problem 14: Longest Collatz sequence

## Problem statement

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1)
contains 10 terms. Although it has not been proved yet (Collatz
Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

## Comments

Seems pretty straightforward: Have a vector of 1 million `0`s, and run
the sequence until you get a value you've already filled in, then you
know how long the sequence is from when you started.

...

Pretty straightforward except, guess what, rust type errors again!
It's nice that the rust compiler is so helpful in its suggestions, but
I'm just doing whatever it tells me without much understanding of why.

Example errors from the compiler:

    error[E0308]: mismatched types
      --> src/main.rs:28:31
       |
    28 |     (1..million).max_by(|x,y| &vec[x as usize].cmp(&vec[y as usize])).unwrap()
       |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       |                               |
       |                               expected enum `std::cmp::Ordering`, found `&std::cmp::Ordering`
       |                               help: consider removing the borrow: `vec[x as usize].cmp(&vec[y as usize])`

    error[E0606]: casting `&u32` as `usize` is invalid
      --> src/main.rs:28:36
       |
    28 |     (1..million).max_by(|x,y| &vec[x as usize].cmp(&vec[y as usize])).unwrap()
       |                                    -^^^^^^^^^
       |                                    |
       |                                    cannot cast `&u32` as `usize`
       |                                    help: dereference the expression: `*x`

    error[E0606]: casting `&u32` as `usize` is invalid
      --> src/main.rs:28:57
       |
    28 |     (1..million).max_by(|x,y| &vec[x as usize].cmp(&vec[y as usize])).unwrap()
       |                                                         -^^^^^^^^^
       |                                                         |
       |                                                         cannot cast `&u32` as `usize`
       |                                                         help: dereference the expression: `*y`

Why on earth am I ending up with references to integers when I'm
iterating over a range? It's not as though it'd make sense to change
the integers.

...

And once I add profiling, it's clearly not worth the cost to try use
`max_by` in any case, and the most straightforward approach seems the
best.
