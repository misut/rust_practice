# Rust Practice(since 2020-01-19)
## Study Note
### Hello, World!(2020-01-19):
- Uses `rustup` to install the latest version of Rust.
- Rust files always end with the _.rs_ extension.
- Uses `rustc` to compile rust files.
- Functions start with a `fn` keyword.
  - Parameters go inside parenthesis, `()`.
  - The function body is wrapped in curly brakets, `{}`.
- Rust uses four spaces rather than a tab.
- Using a `!` means calling a macro instead of a function.
  - `println!` is a macro that prints a string to the screen.
- Strings use double quotes, `"string"`.
- Uses `;` to end lines

### Hello, Cargo!(2020-01-19):
- Cargo is Rust`s build system and package manager.
- Uses `cargo` to make a new project.
  - `cargo build` to build a cargo project.
    - `--release` option does a compilation with some optimizations.
  - `cargo run` to build and run.
  - `cargo check` to check a project without generating a binary.

### Guessing Game(2020-02-15):
- `prelude` library is automatically included to every rust program while compiling them.
- To use other types or functions not in the prelude, they must be brought with a `use` statement.
  - `io` library comes from the standard library `std` and does input/output functions.
- To create a variable, use a `let` statement.
- In Rust, variables are immutable by default.
  - Use `mut` before the variable name to make a variable mutable.
- `//` syntax starts a comment until the end of the line.
- `String` type is provided by the standard library.
  - UTF-8 encoded bit of text.
- `::` syntax indicates that an associated function follows.
- Calls `read_line` to get input from the user.
- `&` indicates that this argument is a reference.
  - References are immutable by default.
  - Uses `&mut` to make it mutable.
- When using the `.foo()` syntax, it is best to divide it.
- `read_line` method returns a value with the `io::Result` type.
  - The `Result` types are enumerations. (`Ok` when successful / `Err` when failed)
  - If the instance of `io::Result` is an `Err` value, `expect` will cause to crash the program and display the message that passed as an argument.
- `{}` is a placeholder for printing values.
- Before using external crates, they must be included into the `[dependencies]` section in `.toml` file.
  - Cargo understands Semantic Versioning.
  - e.g) `^0.5.5^ means anyversion that has a public API compatible with version 0.5.5.
- With `cargo doc --open`, documentation is built and  provided by all of dependencies within it.
- `std::cmp::Ordering` type is another enum. (Less, Greter, and Equal)
  - The `cmp` method compares two values.
- A `match` expression is made up of arms.
  - Each arm consists of a pattern, the `match` looks through each arm's pattern in turn.
- Rust has a strong, static type system, but it also has type inference.
- Rust allows to shadow the previous value with a new one.
- The `parse` method on strings parses a string into some kind of number.
- The `loop` keyword creates an infinite loop.
- Adds `break` to quit the loop.
