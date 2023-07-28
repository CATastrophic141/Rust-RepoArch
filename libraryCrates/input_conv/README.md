# Input Conversion Crate

## This crate contains a simple library used for reading user input and saving the input to a variable with of a given type.

## This crate intends to simplify the way input from a user is read, avoiding the reapeated handling and parsing of simple, single-type inputs.

### This is a introductory effort for rust crate publication, and may be unoptimized. *Use for official projects is unadvised.*

---

### Version History

Version 1.0.0: Added basic library for standard user inputs

Version 1.0.1: Updated README

Version 1.1.0: Added character vector and non-trimmed string return types. Added integer handling of improper decimals and '-' signs.

Version 1.1.1: Optimized sizing

---

```rust
//Function names and return types:

pub fn read_string() -> String

pub fn read_charvec() -> Vec<char>

pub fn read_string_notrim() -> String

pub fn read_u32() -> u32

pub fn read_i32() -> i32

pub fn read_u64() -> u64

pub fn read_i64() -> i64

pub fn read_f32() -> f32

pub fn read_f64() -> f64

pub fn read_u8() -> u8

pub fn read_char() -> char

```

---

Potential updates:

File IO handling. Will implement when required instance appears.
