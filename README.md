# YACC

[![Build Status](https://travis-ci.com/spenserblack/yacc.svg?branch=master)](https://travis-ci.com/spenserblack/yacc)

Yet Another Coloring Crate

# What's the Difference?

This crate provides macros for coloring strings, allowing ANSI colors to be set at compile time instead of runtime.

# Example

```rust
use yacc::println;

println!("It {;green}!", "works");
println!("Warning: {:?;yellow}!", Err(()));
```
