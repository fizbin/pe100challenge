# Problem 25: 1000-digit Fibonacci number

## Problem Statement

The Fibonacci sequence is defined by the recurrence relation:

> F<sub>n</sub> = F<sub>n−1</sub> + F<sub>n−2</sub>, where
> F<sub>1</sub> = 1 and F<sub>2</sub> = 1.

Hence the first 12 terms will be:

> F<sub>1</sub> = 1<br>
> F<sub>2</sub> = 1<br>
> F<sub>3</sub> = 2<br>
> F<sub>4</sub> = 3<br>
> F<sub>5</sub> = 5<br>
> F<sub>6</sub> = 8<br>
> F<sub>7</sub> = 13<br>
> F<sub>8</sub> = 21<br>
> F<sub>9</sub> = 34<br>
> F<sub>10</sub> = 55<br>
> F<sub>11</sub> = 89<br>
> F<sub>12</sub> = 144<br>

The 12th term, F<sub>12</sub>, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to
contain 1000 digits?

## Comments

I once did this problem by [pencil-and-paper and lookups in printed
tables of
logarithms](https://drive.google.com/file/d/1bdCKIWdiQSWf1D-PeaO2TMZ7FIi1zrdd/view)
(in other words, the way you'd need to do this in, say, the 1950s
before hand-held digital calculators).

I think that for this one I'll just use `rug` and the straightforward
Fibonacci implementation rather than the clever solution that I used
with the pencil-and-paper version.

...

Yeah, pretty straightforward. The borrow checker did yell at me, but
it was pretty easy to work around that since `rug` has a sensible set
of methods using references.
