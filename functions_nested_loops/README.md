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
- **jack_bauer()** - prints every minute of the day of Jack Bauer, starting from `00:00` to `23:59`.
```rust
$ rustc functions.rs
$ ./functions | head
00:00
00:01
00:02
00:03
00:04
00:05
00:06
00:07
00:08
00:09

$ ./functions | tail
23:50
23:51
23:52
23:53
23:54
23:55
23:56
23:57
23:58
23:59

$ ./functions | wc -l
1440
```
- **nine_times_table()** - prints the `9` times table, starting with `0`.
```rust
$ rustc functions.rs
$ ./functions
0,  0,  0,  0,  0,  0,  0,  0,  0,  0
0,  1,  2,  3,  4,  5,  6,  7,  8,  9
0,  2,  4,  6,  8, 10, 12, 14, 16, 18
0,  3,  6,  9, 12, 15, 18, 21, 24, 27
0,  4,  8, 12, 16, 20, 24, 28, 32, 36
0,  5, 10, 15, 20, 25, 30, 35, 40, 45
0,  6, 12, 18, 24, 30, 36, 42, 48, 54
0,  7, 14, 21, 28, 35, 42, 49, 56, 63
0,  8, 16, 24, 32, 40, 48, 56, 64, 72
0,  9, 18, 27, 36, 45, 54, 63, 72, 81

$ ./functions | cat -e
0,  0,  0,  0,  0,  0,  0,  0,  0,  0$
0,  1,  2,  3,  4,  5,  6,  7,  8,  9$
0,  2,  4,  6,  8, 10, 12, 14, 16, 18$
0,  3,  6,  9, 12, 15, 18, 21, 24, 27$
0,  4,  8, 12, 16, 20, 24, 28, 32, 36$
0,  5, 10, 15, 20, 25, 30, 35, 40, 45$
0,  6, 12, 18, 24, 30, 36, 42, 48, 54$
0,  7, 14, 21, 28, 35, 42, 49, 56, 63$
0,  8, 16, 24, 32, 40, 48, 56, 64, 72$
0,  9, 18, 27, 36, 45, 54, 63, 72, 81$

$ ./functions | tr ' ' . | cat -e
0,..0,..0,..0,..0,..0,..0,..0,..0,..0$
0,..1,..2,..3,..4,..5,..6,..7,..8,..9$
0,..2,..4,..6,..8,.10,.12,.14,.16,.18$
0,..3,..6,..9,.12,.15,.18,.21,.24,.27$
0,..4,..8,.12,.16,.20,.24,.28,.32,.36$
0,..5,.10,.15,.20,.25,.30,.35,.40,.45$
0,..6,.12,.18,.24,.30,.36,.42,.48,.54$
0,..7,.14,.21,.28,.35,.42,.49,.56,.63$
0,..8,.16,.24,.32,.40,.48,.56,.64,.72$
0,..9,.18,.27,.36,.45,.54,.63,.72,.81$
```
- **print_to_98(number)** - prints all natural numbers from `n` to `98`, followed by a new line.
```rust
$ rustc functions.rs
$ ./functions
number is -10
60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98
```
- **print_times_table(number)** - print the times table for `number`
```rust
$ rustc functions.rs
$ ./functions
number is 12
0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0
0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12
0,   2,   4,   6,   8,  10,  12,  14,  16,  18,  20,  22,  24
0,   3,   6,   9,  12,  15,  18,  21,  24,  27,  30,  33,  36
0,   4,   8,  12,  16,  20,  24,  28,  32,  36,  40,  44,  48
0,   5,  10,  15,  20,  25,  30,  35,  40,  45,  50,  55,  60
0,   6,  12,  18,  24,  30,  36,  42,  48,  54,  60,  66,  72
0,   7,  14,  21,  28,  35,  42,  49,  56,  63,  70,  77,  84
0,   8,  16,  24,  32,  40,  48,  56,  64,  72,  80,  88,  96
0,   9,  18,  27,  36,  45,  54,  63,  72,  81,  90,  99, 108
0,  10,  20,  30,  40,  50,  60,  70,  80,  90, 100, 110, 120
0,  11,  22,  33,  44,  55,  66,  77,  88,  99, 110, 121, 132
0,  12,  24,  36,  48,  60,  72,  84,  96, 108, 120, 132, 144

$ ./functions.exe | tr ' ' . | cat -e
number.is.12$
0,...0,...0,...0,...0,...0,...0,...0,...0,...0,...0,...0,...0$
0,...1,...2,...3,...4,...5,...6,...7,...8,...9,..10,..11,..12$
0,...2,...4,...6,...8,..10,..12,..14,..16,..18,..20,..22,..24$
0,...3,...6,...9,..12,..15,..18,..21,..24,..27,..30,..33,..36$
0,...4,...8,..12,..16,..20,..24,..28,..32,..36,..40,..44,..48$
0,...5,..10,..15,..20,..25,..30,..35,..40,..45,..50,..55,..60$
0,...6,..12,..18,..24,..30,..36,..42,..48,..54,..60,..66,..72$
0,...7,..14,..21,..28,..35,..42,..49,..56,..63,..70,..77,..84$
0,...8,..16,..24,..32,..40,..48,..56,..64,..72,..80,..88,..96$
0,...9,..18,..27,..36,..45,..54,..63,..72,..81,..90,..99,.108$
0,..10,..20,..30,..40,..50,..60,..70,..80,..90,.100,.110,.120$
0,..11,..22,..33,..44,..55,..66,..77,..88,..99,.110,.121,.132$
0,..12,..24,..36,..48,..60,..72,..84,..96,.108,.120,.132,.144$
0,  12,  24,  36,  48,  60,  72,  84,  96, 108, 120, 132, 144$
```
