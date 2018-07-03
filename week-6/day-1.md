# Week 6 Day 1 Notes

Yesterday was a lot of fun! I spent the morning helping give people tours of
the space, and got some lunch with new friends! In the evening, I also paired
with Lucy on some Project Euler problems, finding solutions in
Haskell. I hadn't written any Haskell before, so it was fun to step outside of
my comfort zone and try to find solutions in a new language.

This morning I also helped run the pair programming workshop with Lydia,
which was a lot of fun. During the pairing session, we worked
with Jordan, trying to find a solution with Erlang. There are some really neat
ideas in Erlang, and I think I might end up trying out some more of that (or
Elixir) sometime during my batch.

Project-wise, yesterday was also a really big day! I've been spending the past
few weeks adding compatibility with ELF/Mach-O binaries to twiggy, a code size
profiler that currently targets WebAssembly. Last week, I reached an important
milestone in this journey, the ability to calculate the size of all of the
subroutines in a binary. After checking in with the project maintainer, they
said that I should open up a PR with the work I've completed.

So, I spent some time making some final touches to my work, and opened up a
PR! Waiting on review still, but there was a little block in my work that I
was especially proud of. I had been trying to figure out how to condense some
nested control flow structures that were making things a little tough to read.
Suddenly, I had a light bulb go off, and I ended up with this:

```rust
let item = match item_kind(self, debug_types, comp_unit)? {
  Some(kind @ ir::ItemKind::Subroutine(_)) => {
    let name = item_name(self, debug_str)?
      .unwrap_or(format!("Subroutine[{}][{}]", unit_id, entry_id));
    let id = ir::Id::entry(unit_id, entry_id);
    DieLocationAttributes::try_from(self)?
      .entity_size(addr_size, dwarf_version, rnglists)?
      .map(|size| ir::Item::new(id, name, size as u32, kind))
  }
  _ => None,
};
```

There's a bunch of really neat things happening here, but I was most excited
about the @ operator, which can be used to bind a variable name to the
optional enum variant!

