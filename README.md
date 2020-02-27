# Rust Practice(since 2020-01-19)
## Study Note
### Hello, World!(2020-01-19):
<details>
  <summary>Click to expand</summary>

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

</details>

### Hello, Cargo!(2020-01-19):
<details>
  <summary>Click to expand</summary>

- Cargo is Rust`s build system and package manager.
- Uses `cargo` to make a new project.
  - `cargo build` to build a cargo project.
    - `--release` option does a compilation with some optimizations.
  - `cargo run` to build and run.
  - `cargo check` to check a project without generating a binary.

</details>

### Guessing Game(2020-02-15):
<details>
  <summary>Click to expand</summary>

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

</details>

### Common Programming Concepts - Data Types(2020-02-27):
<details open>
  <summary>Click to expand</summary>

- Rust has a set of _keywords_ that are reserved for use by the language only.
  - Link: [Reference](https://doc.rust-lang.org/book/appendix-01-keywords.html)
- Constants are not variables.
  1. It is not allowed to use `mut` with constants.
  2. When a constant is declared using the `const` keyword, its type _must_ be annotated.
  3. Constants may be set only to a constant expression, not the result of a function call or whatever.
- Rust's naming convention for constants is to use all uppercase with underscores between words.
- Underscores can be inserted in numeric literals to improve readability.
  - e.g) `const MAX_POINTS: u32 = 100_000;`
- Using shadowing, new variable with the same name can be declared in the different type.
- Data types are divided into two subsets which are scalar and compound.
- Rust is a _statically typed_ language.
- A _scalar_ type represents a single value.
  - Integers, floating-point numbers, booleans, and characters.
- The integer type should be annotated like `[SIGN][BITS]`
  - e.g) `u32` for unsigned 32-bit long integer, `i128` for signed 128-bit long integer.
  - The default integer is `i32`.
- Integer literals can be written in any of the forms shown below.

| Number literals | Example       |
| :-------------- | ------------- |
| Binary          | `0b1111_0000` |
| Octal           | `0o77`        |
| Decimal         | `98_222`      |
| Hex             | `0xff`        |
| Byte(`u8` only) | `b'A'`        |

- All number literals except the byte allow a type suffix(`57u8`) and `_` as a visual seperator.
- When compiling in debug mode, Rust checks for integer overflow that cause the program to _panic_ at runtime.
  - _Panic_ means that a program exits with an error.
- When compiling in release mode with the `--release` flag, Rust just performs _two's complement wrapping_.
  - With the library `Wrapping`, it becomes available to wrap integers explicitly.
- Rust's floating-point types are `f32` and `f64`.
  - The default type is `f64`.
- Rust supports the basic mathematical operations.
  - `+` for addition, `-` for subtraction, `*` for product, `/` for division, and `%` for remainder.
  - Link: [All operators](https://doc.rust-lang.org/book/appendix-02-operators.html)
- A boolean type in Rust has two possible values: `true` and `false`.
  - Booleans are _one byte_ in size.
  - Booleans are specified using `bool`.
  - e.g) `let f: bool = false;`
- A character type in Rust is used to represent letters.
  - Characters are _four bytes_ in size and represents a Unicode Scalar Value.
  - Characters are specified using `char`.
  - Character literals are specified with single quotes, as opposed to string literals, which use double quotes.
  - e.g) `let c: char = 'z';`
- _Compound types_ can group multiple values into one type.
  - Rust has two primitive compound types: tuples and arrays.
- Tuples have a fixed length.
  - Once decleared, they cannot grow or shrink in size.
- The tuple is specified with a comma-separated list of values inside parentheses.
  - e.g) `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- Rust supports pattern matching to destructure a tuple value.
  - e.g) `let (x, y, z) = tup;`
- Rust also supports direct access to a tuple element by using a period.
  - e.g) `let six_point_four = tup.1;`
- Arrays also have a fixed length.
  - Every element of an array must have the same type.
- The array is annotated with values of the same type inside square brackets.
  - e.g) `let a: [i32; 5]`
- Array data are allocated on the stack.
- An element of an array can be directly accessed with an index inside square brackets.
  - e.g) `a[0]`
- Rust panics at _index out of bounds_ in runtime.

</details>
