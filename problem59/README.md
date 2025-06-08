# Problem 59: XOR Decryption

## Problem Statement

<p>Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.</p>
<p>A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value, taken from a secret key. The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.</p>
<p>For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes. The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.</p>
<p>Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key. If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message. The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.</p>
<p>Your task has been made easy, as the encryption key consists of three lower case characters. Using <a href="https://projecteuler.net/resources/documents/0059_cipher.txt">0059_cipher.txt</a> (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the original text.</p>

## Comments

I started this one and then put the thing aside, and I'm coming back to it multiple years
later. In the mean time I have forgotten more rust than I thought I ever knew.

The basic idea is to take the frequencies of the 10 most common english letters, and compare
the distribution we get of those letters when we set the key to some particular value. This
comparison is done by the method `l2_distance`, which takes the appropriate distance of
one distribution from some other goal distance.

When I came back to this, I fought a lot with basic rust syntax trying to get `read_csv_to_u8`
to work; I think that there has to be some way to do it without calling `read_to_string` first,
but I guess since we'll end up al least making little `str` slices everywhere to parse doing
a one-time string conversion isn't so bad.