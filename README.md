# sort_cargo_errors
A tool that launches Cargo and reverses the order of errors and warnings

## Purpose
When the Cargo tool, used for Rust-language programming, emits its error messages and warnings, it emits them in the order of appearance. Usually, programmers should to look at such messages in that same order. Though, when running Cargo from the command line, it is much easier to look at the messages backward, starting from the end.

Therefore, there is the need to emit messages in reverse order, so that the most interesting ones are at the end. To do that, all the messages must be captured before emitting them. To show the progress of the compilation, the main line of every message should be displayed also when they are generated.

## Solution
This program launches the Cargo program, applying to it all the command line arguments it receives, and so it a Cargo wrapper. All the bytes output by Cargo or its launched application toward standard error and standard output are captured. The lines emitted to standard output, those emitted by the command `cargo test`, are emitted immediately, and so they are in the usual order. Instead, the lines emitted to standard error,those emitted by the commands `cargo build`, `cargo clippy`, and other commands, are processed in the following way.

The initial lines that show the compilation of other crates, and the lines that begin with a letter are emitted immediately, to show the progress of the compilation. In addition, any empty line marks the end of a message. Such multi-line nessages are collected and, when the compilation is finished, are emitted in reverse order.

The application can, and should, be renamed, for ease of use, but avoiding to name it `cargo` or as any other installed application.
