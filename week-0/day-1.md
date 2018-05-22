### Today's To-do List

[ ] - __a:__ Research how ideas for how `println!` debugging could work in `wasm`
[.] - __b:__ Learn about Rust closures
[.] - __c:__ Put together a brief talk on `impl Trait` w/ description and examples
[x] - __d:__ Notes on today's pair programming workshop
[x] - __e:__ Finish working on `twiggy` issue #58

## a. Println! Debugging in WebAssembly

First, it is worth looking into how we would manually import and use the
`console.log` function from within a WebAssembly binary. One potential
route would entail importing this by default, or if a specific compilation
flag is given to `rustc`. Once this was set, errors would be sent to the
JS console.

## Pair Programming Workshop Notes

For the pair programming workshop, we spent some time working on the Hamming
Distance problem, implementing our solution in Rust. While we were working
on this,  I learned about two really neat things.

__Zipping Iterators__

The first is the `zip` method for the `Iterator` object. This can be used to
'zip' two iterators together, so that each step involves a pair of elements.
This is profoundly useful if you are working with two different sequences.

It is worth noting that once one of these iterators returns `None`, the
iterator will return `None` for all subsequent calls to `next`. Here is a
basic example of what using the `zip` method looks like.

__Entr__

The `entr` command is used to run arbritrary commands when files change.
This is a really useful trick to know about for my Rust workflow, because
I can use this to rebuild and/or test my program after making edits to
a file. No more typing `cargo build` and `cargo test` over and over!

`ls src/*.rs | entr -cr cargo test`

### Late Morning

After I wrapped up the pair programming work and finished up some of the notes
on what I learned while working on that, I completed some of the work that I
had started earlier this morning adding regex functionality to a sub-command
for `twiggy`.

Assuming this all looks good, I might try and use that as a nice starter issue
to pair with someone else on, and look into some of the less trivial issues
that could be tackled.

Adding compatibility with other binary formats seems like it would be an
interesting project, so I might see/ask what that would entail.

After some upstream commits were merged, I also got to learn more about
rebasing. Specifically the `git rebase --continue` command, which will continue
through the rebasing process after all of the conflicts have fixed.

### Afternoon

I started taking some notes regarding how closures and impl trait work, while
I waited for review notes on the work I did to add a new feature to `twiggy`.
I also reached out about the potential project of adding ELF compatibility,
I will look into opening a new issue for tracking and discussing that work.

I got some notes back, so the next step will be to fix those details up,
and update my PR.

### ACM Problem

As the afternoon started to wind down, I tackled another ACM problem with
my friend Avery. This was fun, albeit a little more challenging than we
initially expected.

