# Problem 30: Digit fifth powers

## Problem Statement

<p>Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:</p>
<blockquote>1634 = 1<sup>4</sup> + 6<sup>4</sup> + 3<sup>4</sup> + 4<sup>4</sup><br>
8208 = 8<sup>4</sup> + 2<sup>4</sup> + 0<sup>4</sup> + 8<sup>4</sup><br>
9474 = 9<sup>4</sup> + 4<sup>4</sup> + 7<sup>4</sup> + 4<sup>4</sup></blockquote>
<p>As 1 = 1<sup>4</sup> is not a sum it is not included.</p>
<p>The sum of these numbers is 1634 + 8208 + 9474 = 19316.</p>
<p>Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.</p>

## Comments

The first thing here is to establish upper/lower bounds: a lower bound
could be `10`, and as for an upper bound 6*9<sup>5</sup> is a
six-digit number (`354294`), so any answer must be less than that.

...

Yeah, nothing too unusual here. Did find the convenient `.pow`
function on most number-like types.
