# Problem 55: Lychrel numbers

## Problem statement

<p>If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.</p>
<p>Not all numbers produce palindromes so quickly. For example,</p>
<p class="margin_left">349 + 943 = 1292,<br />
1292 + 2921 = 4213<br />
4213 + 3124 = 7337</p>
<p>That is, 349 took three iterations to arrive at a palindrome.</p>
<p>Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).</p>
<p>Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.</p>
<p>How many Lychrel numbers are there below ten-thousand?</p>
<p class="smaller">NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.</p>

## Comments

In theory, reversing a number and adding it to itself could approximately
double it. Doing that 50 times means potentially multiplying by
2<sup>50</sup>, but with the original number under 10K, that all still
fits in a u64. So I'll just use `u64` values and try each one up to 50
times. I may see if I can reverse a number without stringifying it.
