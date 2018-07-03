# Week 6 Day 0 Notes

Spent some time on Sunday in the space working more on cleaning up the
subroutine size calculation code for twiggy. My idea was following a nice
aphorism that was given to me by Henry, "make it work, make it pretty, make it
fast." Luckily Rust will take care of that last point, so I just worried about
taming a big nest of nightmarish match statements and sequential else if let
blocks. By the end of the day, I felt really happy with the architecture of my
code.

Other time was spent learning about some of the finer points of the DWARF
specification. It turns out that I was misunderstanding some of the location
attributes, which is a good mistake to catch before I (hopefully) open up a PR
containing the past few weeks of work.

I also learned more about what tags and attributes I will need to consider in
order to construct a call graph representing the control flow of the binary. A
highlight was finding a footnote for an attribute that read "this is useful
for people building a call graph." Hey! That's what I'm doing! :smiley:

