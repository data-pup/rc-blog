# Week 1 Day 5 Notes

I came in over the weekend to work a little more on fixing some of the issues
in the way I was calculating size for units in a binary using the DWARF
information. Rather than using the `DW_AT_byte_size` attribute, I should be
instead using memory addresses.

