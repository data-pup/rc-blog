# Week 1 - Day 1 Notes

Today has been spent working on adding ELF and Mach-O compatibility to
`twiggy`. I spent my morning reading through some of the examples included
with the `gimli` crate, as well as an `addr2line` clone that is implemented
using this crate.

I added some scaffolding code to the project, added the new dependencies that
will be required, and ran into an issue while figuring out how to get a
specific section, namely the `.debug_info` section out of the binary.

For now I am also staying away from the problem of cross compilation that
I ran into yesterday. I have a test case that now fails, for the sake of TDD
design. I do not yet have an expectation file, but this will be added once
I have some basic proof of concept working.

I spent some time talking with my friends Ashley and Daniel about music that
we have been listening to, what they were working on themselves, and making
some coffee together.

After getting most of my initial scaffolding into place, I reached out for
a little bit of guidance on what my next steps should be on this. From there,
I decided to context switch and get back to work on some of the Cretonne work
that I had started yesterday.

Hopefully, I can get to a similar point with this, and reach out for some
advice on where to go forward with that work. Then, I should hopefully have
some answers and further guidance on each of these projects :D

This led to some interesting considerations, because Python and Rust have
some fundamentally different types in their standard library. Specifically,
regarding how to represent the arms in a match statement.

