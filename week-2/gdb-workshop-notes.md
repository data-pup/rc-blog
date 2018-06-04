Before you can use gdb, be sure that you have compiled a program including
debug flags. For `gcc` this means using the `-g` flag. If no debuging symbols
are included in a binary, you will be warned by `gdb`.

If we want to debug something, we can start the debugger using `gdb $EXECUTABLE`.

*  `b main` - Add a breakpoint at the main function.
*  `r` - Run the program. (We could provide arguments as well)

The difference between `n` and `s` is that `n` will step into a function.

The `list` command will list the source code, which can help you orient
yourself within the program.

`backtrace`, or `bt` will print the backtrace!

If you need to set breakpoints in other files, you can use a command like:

`b foo.c::15` <- This would add a breakpoint in `foo.c` at line 15.

Say you want to print out an array, you can print specific elements in the
array using syntax like: `p *array@len`. You can also do this with any
contiguous chunk of memory.

`command` can be used for adding commands to breakpoints.

