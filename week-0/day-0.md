# Week 0 - Day 0

Today was the first day at RC. We covered the social rules of the space, met
others attending the Spring batch, as well as previous alumns. They gave some
advice regarding our time here, and we spent time meeting and greeting other
people attending.

Everybody is so nice so far!

I am starting my week of by continuing to help contribute to `twiggy`. `twiggy`
is a code size profiler implemented in Rust. This can be used to help find
sources of code bloat, dead code, call paths, and dominator trees for
WebAssembly (wasm) modules.

The issue that I am working on currently involves adding regexes to the
arguments of a sub-command. This will mean that `twiggy` will be able to
accept a pattern rather than a specific name, and the results will include all
of the items that match a given pattern.

This led to an issue after realizing that using -all- arguments as regex
patterns would cause problems with the existing test cases. At this point,
I should either change the existing test cases, or only use regexp patterns
if a specific flag is included. I am currently leading towards the latter,
but this will require some further investigation.

After I ran into this issue, I spent some time pair programming solving some
of the ACM competition problems.

Tomorrow will include a pair programming workshop at 11 AM that I am excited
for.

