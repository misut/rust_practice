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
  - e.g) `^0.5.5^ means any version that has a public API compatible with version 0.5.5.
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
<details>
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
- Rust also supports direct access to a tuple element by using a period, `.`.
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

### Common Programming Concepts - Functions(2020-02-27):
<details>
  <summary>Click to expand</summary>

- Function definitions start with the `fn` keyword and have a set of parentheses after the function name.
- Rust code uses _snake case_ as the conventional style for function and variable names.
- Rust doesn't care where functions are defined.
- Functions can also be defined to have _parameters_, which are special variables that are part of a function's signature.
- In function signatures, The type of each parameter _must_ be declared.
- _Statements_ are instructions that perform some action and do not return a value.
  - e.g) `let y = 6;`, function definitions
- _Expressions_ evaluate to a resulting value.
  - Expressions do not include ending semicolons.
  - e.g) math operations, calling a function/macro, `{}`
- The return value of a function must be declared with its type after `->`.
- It can be either using the `return` keyword or the final expression in the block to pass the return value.

</details>

### Common Programming Concepts - Control Flow(2020-02-27):
<details>
  <summary>Click to expand</summary>

- An `if` expression allows to branch the code depending on conditions.
  - `if` expressions start with the keyword `if`, which is followed by a condition.
- Conditions of `if` expressions must be provided as Boolean type.
- Rust only executes the block for the first true condition.
- It is more powerful to use `match` than using too many `else if` expressions.
- Because `if` is an expression, it can be placed on the right side of a `let` statement.
  - e.g) `let number = if condition { ... }`
  - In this case, types of last expressions in `if`, `else if`, and `else` blocks must be equal.
- Rust has three kinds of loops: `loop`, `while`, and `for`.
- The `loop` keyword makes an infinite loop.
  - To break out of a loop, the `break` keyword must be placed.
  - The loop can also return values using the `break` expression.
- The `while` loop has a condition, and it checks the condition by each loops until it became false.
- Using `for` loop, it is able to loop through each element of a collection without consideration to the bound.
  - It is more safe to use `for` than to use `while` while looping through a collection.
- `Range` is a type provided by the standard library.
  - It generates all numbers in sequence starting from one number and ending before another number.
  - e.g) (1..4)

</details>

### Ownership(2020-03-07):
<details>
  <summary>Click to expand</summary>

- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change must be stored on the heap.
  - An enough spot on the memory is _allocated on the heap_.
- Ownership rules:
  1. Each value in Rust has a variable that's called its _owner_.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.
- A scope is the range within a program for which an item is valid.
- The _string_ type is stored on the heap.
  - String literals are stored on the stack.
  - e.g) `let mut s = String::from("hello");`
- The double colon, `::`, is an operator that allows to namespace methods under the type.
- That kind of string can be mutated.
  - e.g) `s.push_str(", world!");`
- The `String` type need to be allocated an amount of memory on the heap.
  - The memory must be requested from the OS at runtime.
    - This can be done by calling `String::from`.
  - This memory must be returned to the OS after use.
    - The memory is automatically returned once the variable goes out of scope.
- When a `String` variable goes out of scope, Rust calls a `drop` function.
- When assigning the stack data of a variable to another variable, Rust makes a copy of this value.
- In case of the `String` type, Rust copies only pointers rather than values.
  - A `String` is made up of three parts, pointer, length, and capacity.
- Rust does a _shallow copy_ and also invalidates the first variable to avoid a _double free_ error.
  - In other words, Rust moves the first variable to the second.
- To copy the `String` deeply, use a common method called `clone`.
  - e.g) `let _s = s.clone();`
- It's possible to return multiple values using a tuple.
  - e.g) `(s, length)`

</details>

### Ownership - References and Borrowing(2020-03-12):
<details>
  <summary>Click to expand</summary>

- To prevent the ownership of a variable to be moved into a different scope, use a reference.
  - The ampersand, `&`, allows to refer to some value without taking ownership of it.
  - To dereference it, use the dereference operator, `*`.
- Although the reference goes out of its scope, the variable is not dropped.
  - Setting references as function parameters is called _borrowing_.
- The value of a reference can be modified only when the reference is set to be mutable.
- There can be only one mutable reference of the variable at a time.
- Rust can prevent _data races_ in this fashion.
- A _data race_ happens when three behaviors occur:
  1. Two or more pointers access the same data at the same time.
  2. At least one of the pointers is being used to write to the data.
  3. There's no mechanism being used to synchronize access to the data.
- It is not possible to have a mutable reference while having an immutable one.
  - Multiple immutable references can exist simultaneously.
  - A reference's scope starts from where it is introduced and continues through the last time that it is used.
- A _dangling pointer_ is that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.
- In Rust, the compiler guarantees that references will never be dangling references.
  - To do so, Rust introduced a new feature, _lifetime_.
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
- The `enumerate` method returns a tuple that has an index and a reference of an item.
- A _string slice_ is a reference to part of a `String`.
  - e.g) `let hello = &s[0..5];`
- String literals are also slices pointing to that specific point of the binary.
  - e.g) `&str`
- Other slices work same as string slices.

</details>

### Struct(2020-03-13):
<details>
  <summary>Click to expand</summary>

- To define a struct, write the keyword `struct` and name the entire struct.
  - Inside curly brackets, names and types of data being grouped together should be described.
- To use a struct after defining it, create an _instance_ of that struct by specifying concrete values for each of the fields.
  - Pairs of the field and the value will be assigned in this form `key: value`.
- To get a specific value from a struct, use dot notation.
  - e.g) `user1.email = String::from("anotheremail@example.com");`
  - c.f) Note that the entire instance must be mutable for values to be changed.
- Rust doesn't allow to mark only certain fields as mutable.
  - If the parameter names and the struct field names are same, the field names can be omitted.
- Using _struct update syntax_, it is easily done to create a new instance of a struct that uses most of an old instance's values.
  - The syntax `..` specifies that the remaining fields not explicitly set have the same value as the given instance.
- Tuple structs have the added meaning the struct name provides but don't have names associated with their fields.
  - e.g) `struct Color(u8, u8, u8);`
- To print out debugging information, the annotation `#[derive(Debug)]` should be added just before the struct definition.
  - The placeholder should be also changed into `{:?}` for single line print or `{:#?}` for multiple line print.
- Link: [Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

</details>

### Struct - Methods(2020-05-28):
<details>
  <summary>Click to expand</summary>

- Methods are defined within the context of a struct(or an enum or a trait object).
  - Their first parameter is always `self`, which represents the instance of the struct the method is being called on.
- To define the method within the context of a struct, write an `impl` block and implement it in this block.
  - Use _method syntax_ to call a method: add a dot followed by the method name, parentheses, and any arguments.
  - e.g) `rect1.area()`
- To have a method that changes the instance, use `&mut self` as the first parameter, otherwise `&self`.
- To have a method that takes ownership of the instance, use `self` as the first parameter.
  - c.f) But this is very rare case.
- Associated functions don't take `self` as a parameter, for they don't have an instance of the struct to work with.
  - e.g) `String::from()`

</details>

### Enum(2020-06-25):
<details>
  <summary>Click to expand</summary>

- To define an enumeration, use `enum` keyword and write the variants of the enum inside curly brakets.
  - e.g) `enum IpAddrKind { V4, V6 }`
- The variants of the enum are namespaced under its identifier so they should be separated by using a double colon.
  - e.g) `let four = IpAddrKind::V4;`
- The variants of the enum can have associated values.
  - e.g) `enum IpAddr { V4(String), V6(String) }`
- Each variant can also have different types and amounts of associated data.
  - e.g) `enum IpAddr {V4(u8, u8, u8, u8), V6(String)}`
- This IpAddr enum is already in use.
- Link: [Enum IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html)
- It is able to define methods on enums using `impl` like on structs.
- Rust doesn't have the null feature.
  - But it does have an enum that can encode the concept of a value being present or absent with the `Option<T>` enum.
  - `<T>` is a generic type parameter.
  - e.g) `enum Option<T> { Some(T), None, }`
- Link: [Enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)

</details>

### Enum - `match` Operator(2020-06-26):
<details>
  <summary>Click to expand</summary>

- The `match` operator allows to compare a value against a series of patterns and then execute code based on which pattern matches.
  - The expression after the `match` keyword ca be any type.
- An arm has two parts: a pattern and some code.
  - Patterns can be made up of literal values, variable names, wildcards, and so on.
- When the `match` expression executes, it compares the resulting value against the pattern of each arm, in order.
- The code associated with each arm is an expression and the resulting value of it gets returned for the entire `match` expression.
- Matches in Rust are _exhaustive_.
  - Every possibility must be covered in order for the code to be valid.
- The `_` placeholder can cover default arms.
- The `()` is just the unit value.
- Link: [`unit` Type](https://doc.rust-lang.org/std/primitive.unit.html)

</details>

### Enum - `if let` Syntax(2020-07-13):
<details>
  <summary>Click to expand</summary>

- The `if let` syntax combines `if` and `let` into a less verbose way to handle values that match just one pattern while ignoring the rest.
- The `if let` and `else` is the same as the `match` block with one arm for the first pattern and another for the rest.
  - It means that the block of code that goes with the `else` is exactly same as the block of code that would go with the `_` case in the `match` expression.

</details>

### Module System - Packages and Crates(2020-07-13):
<details open>
  <summary>Click to expand</summary>

- A _crate_ is a binary or library.
  - The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of the crate.
- A _package_ is one or more crates that provide a set of functionality.
  - A package contains a _Cargo.toml_ file that describes how to build those crates.
- Rules for what a package can contain:
  1. A package _must_ contain zero or one library crates, and no more.
  2. A package can contain as many binary crates as possible, but at least one crate(either library or binary).
- Cargo follows a convention that _src/main.rs_ is the crate root of a binary crate with the same name as the package.
  - If the package directory contains _src/lib.rs_, Cargo recognizes _src/lib.rs_ as its crate root.
- A package can have multiple binary crates by placing binaries in the _src/_ directory: each file will be a separate binary crate.

</details>
