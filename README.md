# The Bacon Programming Language

Bacon is an [esoteric programming language](http://en.wikipedia.org/wiki/Esoteric_programming_language), which
uses Bacon.

### Commands

There are eight commands like [Brainfuck](http://en.wikipedia.org/wiki/Brainfuck):

phrase|meaning|
--------|---------|
`Bacon+`|increment the byte at the data pointer.|
`Bacon-`|decrement the byte at the data pointer.|
`Bacon.`|output the byte at the data pointer.|
`Bacon>`|increment the data pointer.|
`Bacon<`|decrement the data pointer.|
`Bacon|`|accept one byte of input.|
`Bacon^`|if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `Bacon?` command.|
`Bacon?`|if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `Bacon^` command.|

### Building

To build, run:
```bash
$ cargo build
```

### Running

To run:
```bash
$ cargo run helloworld.bacon
```
