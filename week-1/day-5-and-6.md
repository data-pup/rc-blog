# Week 1 Day 5 Notes

I came in over the weekend to work a little more on fixing some of the issues
in the way I was calculating size for units in a binary using the DWARF
information. Rather than using the `DW_AT_byte_size` attribute, I should be
instead using memory addresses.

I took some initial notes, and identified where these changes should be made.
Before I take further action though, I should read up on the attributes that
I will use for this, and maybe spend some time reading similar code in
the `bloaty` project, which serves a similar purpose.

---

Spent the day reading through the DWARF v4 specification. Learning a lot, and
getting a better handle on what the different kinds of DIEs representing a
program might contain and look like.

There are some interesting points of nuance here, but I think I should be able
to have a solid understanding of what the process will be for finding the size
of an entity in an object file.

