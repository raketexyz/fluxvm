# FluxVM
literally a virtual machine

## Architecture
### Instruction Set
Mnemonic | Code | Description
:------- | :--- | :----------
halt     | `0`  | Stop program execution.
iconst *value* | `1` | Push an integer to the stack.
iadd | `2` | Perform integer addition on the two top elements of the stack.
syscall *id* | `3` | Make a system call.

### System calls
Code | Usage | Description
:--- | :--- | :----------
`1` | write *fd*, *buf*, *count* | writes up to *count* bytes from the buffer starting at *buf* to the file referred to by the file descriptor *fd*.
`2` | read *fd*, *buf*, *count* | attempts to read up to *count* bytes from file descriptor *fd* into the buffer starting at *buf*.
`3` | open *path*

### Binary format
Binaries consist of a header section, a program section, and a data section.

#### Header section
The header section contains information about the binary.

1. Magic number (`0xf10f0a00`)
2. Reserved field: must be `0x00000000`.
3. Entry point

```
00: f1 0f 0a 00  # Magic number
04: 00 00 00 00  # Version
08: 00 00 00 0c  # Entry point
```

#### Program section
The program section contains a list of instructions.

```
0c: 00 00 00 01  # iconst
10: 00 00 00 01  #   fd for standard out
14: 00 00 00 01  # iconst
18: 00 00 00 30  #   address of string
1c: 00 00 00 01  # iconst
20: 00 00 00 0e  #   length of string
24: 00 00 00 03  # syscall
28: 00 00 00 01  #   write
2c: 00 00 00 00  # halt
```

#### Data section
The data section contains arbitrary data.

```
30: 48 65 6c 6c  Hell
34: 6f 2c 20 57  o, W
38: 6f 72 6c 64  orld
3c: 21 0a        !.
```
