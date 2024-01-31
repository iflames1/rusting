# Rust Variables

Welcome to the Rust Variables guide! 🦀

## Overview

In Rust, variables are used to store and manipulate data. They come with a set of rules and features that ensure memory safety and prevent common programming errors.

## Declaring Variables

Rust allows you to declare variables using the `let` keyword. Here's a basic example:

```rust
let x = 42; // Immutable variable
let mut y = 24; // Mutable variable
```

In Rust, variables are immutable by default. Adding `mut` makes the variable mutable, allowing its value to be changed.

## Types
Rust is a statically-typed language, which means the type of a variable must be known at compile time. You can explicitly specify the type or let Rust infer it:

```rust
let age: u32 = 30; // Explicit type annotation
let height = 5.9; // Type inference (f64 in this case)
```

## Constants
Constants are declared using the `const` keyword. They must have a specified type and can be declared in any scope, including global:

```rust
const PI: f64 = 3.14;
```

## Shadowing
Rust allows shadowing, where you can declare a new variable with the same name as an existing one. This is useful for changing the type or reusing the variable name:

```rust
let count = 5;
let count = count + 1; // Shadowing
```

## Printing Variables
You can use the `println!` macro to print variables to the console:

```rust
let name = "Rust";
println!("Hello, {}!", name);
```

## Conclusion
Understanding variables in Rust is crucial for writing safe and performant code. Whether immutable or mutable, variables play a central role in data manipulation.
