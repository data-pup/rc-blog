# Week 1 - Day 0 Notes

My second week (yes I am 0-indexing my blogging notation) started today.
I was first into the space today, which gave me some nice extra time to brew
some coffee, catch up on some Zulip discussions, write up a checkin message
regarding the work I got done on Friday, put together some notes on the next
projects that I will be working on this week, and listen to some techno!

My two main projects are basically in swing now, so I am now working on
adding ELF/Mach-O compatibility to `twiggy`, and helping to refactor some
of `cretonne`'s internal DSL used for describing target ISAs into Rust.

Aside from that, I am also planning to spend some time organizing some
questions/problems for people to work on for the language learning event that
I am putting on this Wednesday :o)

Today I'll largely be dividing my time between the two `twiggy` and `cretonne`
issues however :)

---

During the early afternoon I worked on the first steps for refactoring one of
the components in the cretonne DSL. There is still a lot of work to be done,
but I learned more about the `format!` macro, and got the bulk of the methods
drafted.

The questions I still have to answer regarding this work relate to testing,
and how to wire in the new source generation file. There are also some
non-trivial types, including a dictionary of tuples, that I will need to
look into further before I can implement.

Another highlight of the day included looking into how to override
dependencies in Cargo, with a friend who was testing patch they had made to
a library that they were using.

After this, I decided to switch over and start working a little bit on twiggy,
and figure out how to handle some of the initial steps regarding adding the
dependencies needed, as well as test fixtures.

