# Problem 46: Goldbach's other conjecture

It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

## Comments

Well, I guess we can first try setting up a bitvec-based 
seive to compute all the primes up to... 1 million? and then
do those fast shifting and bitwise oring of bitvecs that we did in
problem 23.

...

Well, I could have used a much smaller upper limit, and there was a short detour to rewrite problem23
using a more modern bitvec version. But still, basically as expected.