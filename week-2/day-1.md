# Week 2 Day 1 Notes

Before I had arrived to RC this morning, I ended receiving a series of
notifications about some `wasm-pack` issues that had been open for a little
while.

I spent some time in communication with the maintainers regarding how to get
those issues taken care of, and what the strategy should be regarding how
to determine what version of the `wasm-bindgen` etc. package to use. One
good point that was raised was that it would be nice if a specific version
could be specified rather than force installing the most recent version.

### To Do

*  Continue working on the twiggy parsing logic.
*  Start working on a `how to contribute to a github project` talk?

Other things I plan on attending doing today include the 'Ignorance Mapping'
meetup, a chance to be open and chart out what we do -not- know. This is a
really neat idea I think, and also provides a chance for people to find
topics that they can teach others.

This evening there is also a viewing party for Star Trek, which should be
a lot of fun! For now however, it is off to work on some twiggy things,
figuring out how to get all of this DWARF information converted into IR items.

---

(Summarizing my day using my check-in the next morning.)

Spent most of yesterday working on more `twiggy` stuff. Getting a better
handle on some of the `Option<T>` methods, and am now very much in love with
using `ok_or(..)?`. I also finally wrapped my head around its `map` method,
which only applies when an optional value contains `Some(val)`.

This means you can use `map` to convert an `Option<T>` to an `Option<U>`,
provided the argument is some function `T -> U`. Composing all these methods
in longer chains was a little confounding at first, but I am feeling much more
comfortable doing so, which is good given that the library I'm working with
often returns `Result<Option<T>, Error>` objects that can be hard to handle
otherwise.

I attended the 'Mapping Ignorance' workshop and had a lot of fun thinking
about some concepts that I would like to study further regarding
asynchronous/concurrent code. The Star Trek viewing was a lot of fun too, I
enjoyed the new series a bunch despite it not having Data (the best Trek
character) in  it.

