# Week 8 Day 1 Notes

Yesterday was nice `:)` In the morning, I was still waiting on some
feedback on my Cranelift PR, so I needed to find another way to spend some
time. I ended up finding this meta-issue on the wasm-bindgen repository:
https://github.com/rustwasm/wasm-bindgen/issues/275

In summary, this relates to ongoing work to expose globally available APIs
included in the ECMAScript standard, through a `wasm_bindgen::js` module. This
sounded like fun, and I hadn't ever learned much about how people make
bindings for language interopt. I found some methods for JavaScript's `String`
class that hadn't been finished yet, so I spent the morning adding these. This
led to two PR's, and they each got merged! :tada:

https://github.com/rustwasm/wasm-bindgen/pull/490 and
https://github.com/rustwasm/wasm-bindgen/pull/493

Then, I spent some time working on the post-lunch prep problems, and was
reminded that Rust isn't always an agile language for constructing ad-hoc data
structures. I actually really like this, and I think that there's probably an
interesting technical talk to do about how the standard library constrains the
collections that are made available to the user. So, I didn't end up solving
the problem, but it was cool to see other people's solutions!

The Rust Study Group was also a ton of fun. It's possibly grown too large to
host in Turing, and I'm really glad so many people have stepped up to present
so far. Hoping this format keeps working out going forward, and that everybody
else has had fun too. :crab: :heart:

Around this time, I ended up getting some wonderfully detailed review notes on
my Cranelift PR, so I pivoted to work on that in the evening. Someone even
reached out via IRC to say that they were happy to see that someone had
stepped up to take this task on! :smile:

After fixing the things that were noted on, I ended up making the code a fair
bit cleaner as well. It turns out, there were some ways that I could simplify
the """inheritance""" (big air quotes there) hierarchy, and I figured out some
neat ways to cast enums with explicit discriminators into numbers, as well as
finding the logaraithm (base-2) of a number without using floating point
arithmetic!

