# Week 3 Day 0 Notes

Continued working on size calculation logic for twiggy over the weekend, and
broke through a wall that I'd been stuck on! The issue that I'd been dealing
with regarded the fact that some entities associated with machine code in a
binary might not occupy a contiguous range of addresses. This means traversing
a lookup table and processing a series of range entries.

In the process of getting this done, I learned that else if let is valid Rust
syntax, which was pretty exciting. This ended up being much cleaner than a
match statement on multiple entities, and writing an if let with a ? operator
in the clause made me feel pretty clever.

At this point I need to figure out how to process subroutines, and figure out
what to do with tags that aren't relevant to the size profiling in the initial
pass for collecting items into the intermediate representation (IR), before I
start implementing the pass to collect graph edges by finding call sites and
instances of subroutine inlining.

Today I am going to try and get some work done on a wasm-pack issue, to
identify whether certain commands are already in the $PATH, and of the correct
version, so that dependencies don't need to be reinstalled every time the
command is run. I might also try and get some other work done on the cretonne
refactoring I started a week or two ago, which will involve learning a little
bit more about build.rs files are used with cargo.

---

In the afternoon I paired with a friend to learn more about Idris, and explore
the concept of type driven development. Idris has a robust type system that
offers a number of interesting features, such as dependent types, and even
deriving code based on the types and available bindings at a given position.

I will be giving a talk tonight that I am excited to give, and pairing on
an SKI combinator (?) in rust's type system.

Aside from that, I am going to try and form a 'Papers We Love' type of event
running here, which I'll be talking with one of the facilitators about today.

I am also going to spend some time diving into the build system for cretonne,
to figure out how i can get my new Rust implementation of the `gen_types`
file to create a `new_types.rs` in the build directory.

---

