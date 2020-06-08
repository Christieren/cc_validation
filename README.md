# CC_Validator

A quick and dirty Credit Card Validator for cards using the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm).

This was just a project to get me more familiar with Rust and it is in no way good code.

This is the [pseudo code](https://en.wikipedia.org/wiki/Luhn_algorithm#Pseudocode_implementation) that I followed for the validation:

```
   int sum := integer(purportedCC[length(purportedCC)-1])
     int nDigits := length(purportedCC)
     int parity := nDigits modulus 2
     for i from 0 to nDigits - 2 {
         int digit := integer(purportedCC[i])
         if i modulus 2 = parity
             digit := digit × 2
         if digit > 9
             digit := digit - 9 
         sum := sum + digit
     }
     return (sum modulus 10) = 0
```
## Known issues
1. Will panic if no input supplied.
2. Will return valid for a string containing the same alpha character(s).