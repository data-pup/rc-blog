# Week 7 Day 1 Notes

...

Worked on both of my projects yesterday, and got a fair amount done!

Checked into why my PR was not passing the CI tests, and it turned out that
there were some small errors with my conditional compilation flags when the
DWARF compatibility feature that I'm adding to the workspace's parsing crate
was not enabled. I realized that I could conditionally include/exclude the
entire submodule I've added, rather than adding `#[cfg(...)]` flags throughout
my code. So fixing that was easy enough, but the real victory was learning how
to locally run the tests used in CI.

At this point, my changes have passed the code review phase, and should be
merged pretty soon! Looking forward to that.

The rest of my day was spent on further work refactoring code responsible for
generating Cretonne's IR types. I added the first of the 'lane' types, which
are scalars that can be used in SIMD operations. By the end of the day, I
could emit all of the boolean types!

In the evening, had a nice conversation with some friends about the tools we
use to program, how we conceptually represent programs, how the actions we try
to automate affect programming languages, what the idea of "readable" code
means, and how these things changing in the future could alter how we pair
program.

Today, I'm hoping to finish the logic for emitting the rest of the Cretonne
lane types.
