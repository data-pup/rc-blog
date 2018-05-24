# Problem: 4Sum II

## Problem Description

Given four lists A, B, C, D of integer values, compute how many tuples
(i, j, k, l) there are such that A[i] + B[j] + C[k] + D[l] is zero.

To make problem a bit easier, all A, B, C, D have same length of N where
0 ≤ N ≤ 500. All integers are in the range of -2^28 to 2^28 - 1 and the result
is guaranteed to be at most 2^31 - 1.

## Notes

First, we should be sure that our integers will not overflow. Given that the
results are guaranteed to be at most 2^31 - 1, we have a pretty good idea that
we can use i32 values to represent the elements and their results.

```
✨  python
Python 2.7.12 (default, Dec  4 2017, 14:50:18)
[GCC 5.4.0 20160609] on linux2
Type "help", "copyright", "credits" or "license" for more information.
>>> 2 ** 31 - 1
2147483647
```

This lines up with the documentation for the `i32` primitive type in the
Rust stdlib documentation:

```
assert_eq!(i32::max_value(), 2147483647);
```

It is also noted that all of the lists will have the same number of elements,
so we can use the `zip` method exposed by `Iterator`.
