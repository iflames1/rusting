# Rust Functions 🦀


## Overview

Functions in Rust are essential building blocks for organizing and structuring your code. They allow you to encapsulate logic, promote reusability, and contribute to a modular codebase.

## Declaring Functions

In Rust, you declare a function using the `fn` keyword, followed by the function name, parameters, return type, and body:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```
In this example, `add` is a function that takes two parameters (`x` and `y` of type `i32`) and returns an `i32`.

## Calling Functions
You call a function by using its name followed by parentheses, passing any required arguments:

```rust
let result = add(3, 5);
```
Here, the `add` function is called with arguments `3` and `5`, and the result is stored in the `result` variable.

## Function Parameters
Rust supports both named and unnamed (tuple) function parameters. You can specify default values and use the `...` syntax for variable-length arguments:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```
In this example, `greet` takes a string slice (`&str`) parameter.

## Return Values
Functions can return values using the `->` syntax:

```rust
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
```
Here, `multiply` returns the product of `x` and `y`.

## Function Visibility
By default, functions are private to the module they are defined in. You can use the `pub` keyword to make them public:

```rust
pub fn public_function() {
    // Code here
}
```
## Conclusion
Understanding functions is fundamental to Rust programming. They enable code organization, reusability, and help create clean and maintainable software.

## [Functions1](./src/functions.rs)
- **print_alphabet_x10()** - prints the alphabet, repeated ten times.
```rust
$ rustc functions.rs
$ ./functions
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxyz
```
- **_is_lowercase(c: char)** - checks for lowercase character.
```rust
$ rustc functions.rs
$ ./functions
The generated character 'K' is not lowercase.
```
