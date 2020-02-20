# Problem 7

## Problem statement

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
see that the 6th prime is 13.

What is the 10 001st prime number?

## Comments

While I could use my `find_prime_factor` routine from problem 2, I
think I'm going to approach this with a seive. It'll require an
overestimate of where the 10,001st prime is, but I think I can do
that.

...

Well, primes are a bit more rare than I thought so I had to go with a
dynamically determined sieve size. Also, this marked my first
encounter with an error from the borrow checker, even though the real
error was just forgetting to declare a variable correctly:

    error[E0596]: cannot borrow `sieve` as mutable, as it is not declared as mutable
      --> src/main.rs:14:5
       |
    13 |     let sieve : Vec<bool> = vec![true; sieve_size];
       |         ----- help: consider changing this to be mutable: `mut sieve`
    14 |     sieve[0] = false;
       |     ^^^^^ cannot borrow as mutable
