# Day 3

### Morning

I had an early start today, and got to the space fairly early. I spent some
time getting a better understanding of how to use Zulip, and reading. My
friend Ross was the second person to arrive today, so we had some time
to talk about how our weeks were progressing.

We talked about unit testing practices, Python libraries that we liked, and
a little bit about Fortran, and then discussed open source contribution. I
showed him CodeTriage, a website that allows you to find open source projects
that need help from new contributors, and I gave some advice on finding issues
that are good starting points by filtering for issues in GitHub that are tagged
as a good first issue, or offer mentorship.

I wrapped up some of the slides that I had been working on for the
presentations tonight, and RSVP'd to some events that are happening today.

After that, I prepared myself to return to the `cretonne` issue that I had
been working on. At this point, there are only a few details to iron out,
and from there I should be about done with that!

I will hopefully try and spend some time researching the ELF compatibility
issue for `twiggy`, and maybe open a tracking issue for this.

I also spent some time talking with some friends about organizing some events
for learning new programming languages, which I will think more about later
today, and/or later this week.

## Late Morning

I submitted the patch for my PR to `cretonne`, and asked some questions in
the IRC channel regarding the CI scripts that are being run. It turns out
that rust-fmt is in a transitional state, so there were some issues when
trying out the new 'train', which is currently on v0.9.

I posted inquiring who else would like to try and learn a new language, and
at this point I might try and spend some time researching the work that would
need to be done for two large projects that I have been thinking about spending
time on here:

*  Adding ELF compatibility to `twiggy`
*  Refactoring the `cretonne` DSL

At this point I am thinking about breaking off for lunch, so I will continue
on that work after I have some food in me!

### Afternoon

At lunch I got learn a little more about the problems involved with
implementing search, especially with regards to how it related to
personalization. I asked about some reading material that would be recommended,
and this might turn into a topic I spend some time studying here :)

I also reached out about the cretonne refactoring work, and noted that I would
not mind trying to help reimplement some of that code in Rust. This might be
involved, but there were some notes on places that would be good to start.

At this point I had.. kind of finished a lot the issues I had hoped to
contribute solutions to for both `cretonne` and `twiggy`, and figured it would
be good to take some time and work on something different and new. Luckily,
there is a study group for working on algorithm problems, which I decided to
jump into.

As a side note regarding something that I have been thinking about, it might
be fun to try and implement a simple clone(ish) of Logstash using Rust. I might
return to this idea, as it has been bouncing around in my head for a little
while. I had previously tried to build a little stream editor earlier on while
learning Rust, but left it in a sorry state of incompletion. I might try
and build this, because I think some of the crates ecosystem could come in
handy here!

