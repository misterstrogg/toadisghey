# The toadisghey Programming Language

toadisghey is an [esoteric programming language](http://en.wikipedia.org/wiki/Esoteric_programming_language), which
uses toadisghey.

### Commands

There are eight commands like [Brainfuck](http://en.wikipedia.org/wiki/Brainfuck):

phrase|meaning|
--------|---------|
`toadisghey+`|increment the byte at the data pointer.|
`toadisghey-`|decrement the byte at the data pointer.|
`toadisghey.`|output the byte at the data pointer.|
`toadisghey>`|increment the data pointer.|
`toadisghey<`|decrement the data pointer.|
`toadisghey|`|accept one byte of input.|
`toadisghey^`|if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `toadisghey?` command.|
`toadisghey?`|if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `toadisghey^` command.|

### Building

To build, run:
```bash
$ cargo build
```

### Running

To run:
```bash
$ cargo run helloworld.toadisghey
```
