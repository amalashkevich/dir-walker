# Dir walker

The function `print_line_count` recursively finds all files with a given file extension in that directory and all sub-directories, and counts the number of lines in the file and prints it to stdout.

It accepts two parameters:

    - The path (e.g. "/Users/am/Documents/dev/rust/")
    - The extension (e.g. "rs")

Here is an output example:

```
"/Users/am/Documents/dev/rust/dir-walker/src/main.rs" 29
"/Users/am/Documents/dev/rust/example1/hello_cargo/src/main.rs" 3
"/Users/am/Documents/dev/rust/example1/src/main.rs" 3
"/Users/am/Documents/dev/rust/guessing_game/src/main.rs" 35
"/Users/am/Documents/dev/rust/vm-web/src/lib.rs" 79
"/Users/am/Documents/dev/rust/vm-web/src/utils.rs" 11
"/Users/am/Documents/dev/rust/vm-web/src/vm_ta.rs" 158
"/Users/am/Documents/dev/rust/vm-web/tests/web.rs" 13
```
