# Week 10 Day 0 Notes

Didn't check in as frequently last week, but I wanted to summarize some of my
victories, and set out some goals for my last two weeks here.

First, the PR I mentioned last week was accepted and merged! I added summaries
for truncated output in one of twiggy's subcommands, and refactored some code
  to get rid of duplicated work, as well as making the logic a little more
  functional. I'd been a little nervous about such an.. *opinionated*
  refactoring, but it received a warm welcome and was merged shortly thereafter.

I also got to take care of some organizational work for cranelift, preparing
the metaprogramming crate for a future reimplementation using procedural
macros. Unfortunately, the project needs to support v1.25, which does not
support this feature, but I helped set everything up so that it would be easy
to do so in the future. Renaming the existing implementation and moving my
work into its place felt pretty neat. :sunglasses:

I spent Friday night learning about 'clippy', which is a linter for Rust.
Having only used linters in dynamically types languages, I was pretty shocked
at how much more a linter can do in a statically typed language. Submitted a
PR to twiggy with some various fixes, which also led me into some parts of the
codebase I hadn't seen before :)

Other activities for the weekend included staying away from computers, going
out to a picnic, and staying up late (dancing).

As for this week, I'm hoping to finish up another PR for twiggy, adding some
features to its 'monos' command for calculating bloat due to generic
function's monomorphizations, making a PR to fix a wasm-pack issue, and
starting work on the next stage of metaprogramming reimplementation for
cranelift. Rust's 2018 edition will also be released, so I might try and learn
some more about its new features.

