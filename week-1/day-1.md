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

