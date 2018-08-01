# Week 10 Day 1 Notes

Currently wallowing in the end of batch blues, but I learned some neat things
yesterday! Specifically, I got a much better handle on Rust's lifetimes
system. My :fire: Hot Take :fire: on lifetimes is that they are a borderline
anti-pattern in many cases. In short, they're often symptomatic of straying
from Rust's standard idioms and evidence that you might be trying to fight the
borrow checker, rather than use the borrow checker.

In my case however, I had a situation where it would be more performant to
delay the process of converting some borrowed string slices into
heap-allocated String values, after these collections have been truncated
according to some input options. This was a little frustrating at first, and
led to some fairly opaque error messages, but I ended up getting this fixed
later on in the day. :smile:

Aside from that, I got some review on a PR I've had open for a few weeks on
wasm-pack that aims to add some version coordination with wasm-bindgen, and
they said this would be a really beneficial feature to add. I have some edits
to make, and will need to rebase it off of the current master branch, but I'm
excited to get this one merged soon! :slight_smile:

