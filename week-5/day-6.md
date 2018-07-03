# Week 5 Day 6 Notes

At this point, the previous batch had just ended!

Took a break from computers yesterday, but I did spend some of yesterday
reading the Rustonomicon, the dreaded tome of unspeakable, eldritch,
arithmetical terrors. I learned a lot about what kinds of undefined behavior
Rust is designed to prevent, how safe/unsafe Rust interact, how data is
organized in memory, and what the Drop actually is.

I really loved this excerpt, this might be my favorite example of
demonstrating why Rust's rules about immutable/mutable borrows exist:

Of course, Rust's story around ownership is much more complicated than just
verifying that references don't escape the scope of their referent. That's
because ensuring pointers are always valid is much more complicated than this.
For instance in this code,

```rust
let mut data = vec![1, 2, 3];
// get an internal reference
let x = &data[0];

// OH NO! `push` causes the backing storage of `data` to be reallocated.
// Dangling pointer! Use after free! Alas!
// (this does not compile in Rust)
data.push(4);

println!("{}", x);
```

Naive scope analysis would be insufficient to prevent this bug, because data
does in fact live as long as we needed. However it was changed while we had a
reference into it. This is why Rust requires any references to freeze the
referent and its owners.

Luckily I'm not currently working with anything that requires unsafe code, but
the model of "safe" and "unsafe" is pretty neat, and it was fun learning more
about it!

Other highlights were finally grokking the difference between trait bounds and
trait objects. Had a fun discussion about this with Daniel F. yesterday, and
what the trade-offs between the two are.

There is a nice discussion of this here: https://doc.rust-lang.org/nomicon/exotic-sizes.html

