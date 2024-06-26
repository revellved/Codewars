# Next smaller number with the same digits (Rust)

> https://www.codewars.com/kata/5659c6d896bc135c4c00021e/rust

## Description:

Write a function that takes a positive integer and returns the next smaller positive integer containing the same digits.

For example:

next_smaller(21) == Some(12)
next_smaller(531) == Some(513)
next_smaller(2071) == Some(2017)

Return -1 (for Haskell: return Nothing, for Rust: return None), when there is no smaller number that contains the same digits. Also return -1 when the next smaller number with the same digits would require the leading digit to be zero.

next_smaller(9) == None
next_smaller(135) == None
next_smaller(1027) == None // 0721 is out since we don't write numbers with leading zeros

    some tests will include very large numbers.
    test data only employs positive integers.

The function you write for this challenge is the inverse of this kata: "Next bigger number with the same digits."
