Thursday and Friday consisted of some more work studying how Rust's traits can
be used to replace OOP inheritance patterns. Gave a technical talk on how
Cargo build scripts work, and took care of some edits to fix comments provided
in my twiggy PR's code review.

One highlight of Friday was learning more about how the features system works
in Cargo. You can conditionally include/exclude code at compilation time to
target specific platforms, or to enable/disable special features. Because the
parsing crate that I'm contributing to exposes a wasm-specific API to some of
the other crates in the workspace, this was required to gate the ELF/Mach-O
features I am introducing. It turned out to be easier than expected, and my
changes were marked as approved after fixing everything! The PR hasn't been
merged yet, but it's almost ready.

