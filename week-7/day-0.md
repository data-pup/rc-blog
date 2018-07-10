# Week 7 Day 0 Notes

Spent most of yesterday working through some bugs in the source code emission
of IR types for Cretonne. While I have now added my modifications to the build
script and have figured out conceptually how my code should be structured to
replace the existing Python code, there were a few minor bugs that I was
noticing in the emitted code itself.

Figuring out how to debug efficiently this took a little effort, because the
standard output of a build script is placed into a special file used by Cargo
to set certain flags. This meant that debugging with print statements was not
an effective method, since nothing is printed, and the file this ends up in
can be a little tedious to find and navigate to.

This was a fun chance however, to come up with a nifty little bash pipeline to
print the contents of the newly generated types file. By wrapping this up with
`ag -l | entr -rc sh -c 'Do stuff here'`, I could repeat this process whenever
I changed a file in the project.

```sh
ag -l | entr -rc sh -c 'ls -t1 target/debug/build/ | grep cretonne-codegen | head -n 1 | xargs -I {} cat target/debug/build/{}/out/new_types.rs'
```

Overall, I was able to fix some bugs related to indentation, uniquely
numbering different IR types, line parsing, added documentation comments to my
dynamically emitted types, and found an insidious little logic bug related to
the fact that iterators are lazily evaluated.

Today, I'll be continuing work on this, at this point I've got one small
category of types working correctly. Now it's just a matter of applying that
work to the other types that can be represented in the IR scheme.

