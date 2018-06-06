# Week 2 Day 2 Notes

More twiggy work today! Not too many notes taken, as I was deep in the DWARF
specification, and working through some design decisions regarding how I
could structure this.

One important takeaway involved working towards a refactor to make the size
of items into a `u64` value. This will not work, as this cannot be converted
into a `f64` value, which is required to make some percentage calculations
in the `analyze` file.

Here is a link to the pertinent code, which I found via the stdlib
documentation.

https://doc.rust-lang.org/src/core/num/mod.rs.html#3995-3997

```
// Note: integers can only be represented with full precision in a float if
// they fit in the significand, which is 24 bits in f32 and 53 bits in f64.
// Lossy float conversions are not implemented at this time.
```

