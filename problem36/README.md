# Problem 36: Double-base palindromes

## Problem statement

<p>The decimal number, 585 = 1001001001<sub>2</sub> (binary), is palindromic in both bases.</p>
<p>Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.</p>
<p class="info">(Please note that the palindromic number, in either base, may not include leading zeros.)</p>

## Comments

The only question here is what the easiest/fastest way to check for
palindromes is. I suppose I can speed things up slightly by ignoring
all even numbers, as those can't be base-2 palindromes.

Though actually... what if I built up the numeric value in one base so
as to be an (odd) palindrome, and then checked the number to see
whether it were also a palindrome in the other base? That would likely
mean visiting the palindromes out of order, but all I care about is
their sum.

...

Well, I think that's a good algorithmic idea, but my attempt to code it
rust failed miserably. Going to go ask the rust community in discord.

Here's what the rust compiler gives me as an error message:

    error: reached the type-length limit while instantiating `palindrome_loop::<[closure@src/m...igh_added:&u32]]]]]]]]]]]]]]]]]>`
      --> src/main.rs:24:1
       |
    24 | / fn palindrome_loop<F>(n_digits: u8, mut found_func: F)
    25 | | where
    26 | |     F: FnMut(u32),
    27 | | {
    ...  |
    39 | |     }
    40 | | }
       | |_^
       |
       = note: consider adding a `#![type_length_limit="2097145"]` attribute to your crate
    
I suspect that I'm somehow creating an arbitrarily long type, and
no `type_length_limit` would help me.
