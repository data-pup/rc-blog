# Week 2 Day 0 Notes

I started out a little late today, after taking care of some extraneous
errands. Once I got to RC, I talked with my friend Lydia, who is interested
in learning more about Rust, and compilers. I spent some time providing some
different learning resources to them, and then gave a brief tutorial on how
to use IRC. Because much of the Rust community is IRC oriented, it can be
a profoundly useful resource for getting advice about Rust :)

Next, I attended a workshop on using `gdb`, hosted by Caroline, who has more
expertise in C/C++ than I. They are doing some really neat projects involving
simulations/visualization with C, and I was happy to learn more about that.
The notes from that session are included in a separate file in this directory.

---

### Goals for Today
*  Add an impl Trait block for compilation units in a file
*  Calculate the size / name of compilation units
*  Find the next structs that I should implement this Parse trait for

---

Today I worked on refactoring my previous work implementing the parsing traits
for `twiggy`. This required a fair bit of rethinking, and some hairy work
involving lifetimes, generics, etc., but I made a lot of progress.

At this point, I have divided everything into smaller individual functions
responsible for parsing a file, its compilation units, and individual
entries. There is still a fair bit of work to do, but this has gotten me
set up to make solid progress on the size calculations tomorrow.

