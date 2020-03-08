# Problem 15 (Lattice paths)

## Problem statement

Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

[*See original image on the Project Euler site*](https://projecteuler.net/problem=15)

How many such routes are there through a 20×20 grid?

## Comments

This is pretty straightforward, just `40!/(20!*20!)`. Maybe I'll be
clever with how I compute that to avoid overflows.

...

Yep. A little hiccup because I couldn't arrange the numbers manually
easily, but pretty straightforward.
