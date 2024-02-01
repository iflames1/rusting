# Rust Variables 🦀

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


## [Functions1](./src/functions1.rs)
### The value for `number` in function parameters here are random numbers.
- **check_signed(number)** - check if a number is signed or not
```rust
$ rustc functions1.rs
$ ./functions1
0 is zero

$ rustc functions1.rs
$ ./functions1
1009 is positive

$ rustc functions1.rs
$ ./functions1
-100 is negative
```
- **last_digit(number)** - checks the last digit of a number
```rust
$ rustc functions1.rs
$ ./functions1
Last digit of 100 is 0 and is 0

$ rustc functions1.rs
$ ./functions1
Last digit of 1009 is 9 and is greater than 5 and not 0

$ rustc functions1.rs
$ ./functions1
Last digit of -1009 is -9 and is less than 6 and not 0
```

- **print_alphabet()** - print the lowercase alphabet
```rust
$ rustc functions1.rs
$ ./functions1
abcdefghijklmnopqrstuvwxyz
```

- **print_alphabet2()** - print the lowercase and uppercase alphabet
```rust
$ rustc functions1.rs
$ ./functions1
abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ
```
- **print_numbers()** - prints all single digit numbers
```rust
$ rustc functions1.rs
$ ./functions1
0123456789
```

- **print_alphabet_reverse()** - prints the lowercase alphabet in reverse
```rust
$ rustc functions1.rs
$ ./functions1
zyxwvutsrqponmlkjihgfedcba
```

- **print_numbers_formatted()** - print numbers from 0-9 formatted with commas
```rust
$ rustc functions1.rs
$ ./functions
0, 1, 2, 3, 4, 5, 6, 7, 8, 9
```

- **print_comb2()** - prints all possible different combinations of two digits
```rust
$ rustc functions1.rs
$ ./functions
01, 02, 03, 04, 05, 06, 07, 08, 09, 12, 13, 14, 15, 16, 17, 18, 19, 23, 24, 25, 26, 27, 28, 29, 34, 35, 36, 37, 38, 39, 45, 46, 47, 48, 49, 56, 57, 58, 59, 67, 68, 69, 78, 79, 89
```
- **print_comb2()** - prints all possible different combinations of three digits
```rust
$ rustc functions1.rs
$ ./functions
012, 013, 014, 015, 016, 017, 018, 019, 023, 024, 025, 026, 027, 028, 029, 034, 035, 036, 037, 038, 039, 045, 046, 047, 048, 049, 056, 057, 058, 059, 067, 068, 069, 078, 079, 089, 123, 124, 125, 126, 127, 128, 129, 134, 135, 136, 137, 138, 139, 145, 146, 147, 148, 149, 156, 157, 158, 159, 167, 168, 169, 178, 179, 189, 234, 235, 236, 237, 238, 239, 245, 246, 247, 248, 249, 256, 257, 258, 259, 267, 268, 269, 278, 279, 289, 345, 346, 347, 348, 349, 356, 357, 358, 359, 367, 368, 369, 378, 379, 389, 456, 457, 458, 459, 467, 468, 469, 478, 479, 489, 567, 568, 569, 578, 579, 589, 678, 679, 689, 789
```
