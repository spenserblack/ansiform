# AnsiForm

[![Crates.io](https://img.shields.io/crates/v/ansiform)](https://crates.io/crates/ansiform)
[![Docs.rs](https://docs.rs/ansiform/badge.svg)](https://docs.rs/ansiform)
![Crates.io](https://img.shields.io/crates/d/ansiform)
[![CI](https://github.com/spenserblack/ansiform/actions/workflows/ci.yml/badge.svg)](https://github.com/spenserblack/ansiform/actions/workflows/ci.yml)

Easily format strings with ANSI colors

This crate provides a macro for coloring strings, allowing ANSI colors to be set at compile time
instead of runtime.

> [!CAUTION]
> There is a very good chance that you do *not* want to set styling at compile-time.
> This can result in styling that was compiled on one system being forced when the
> executable is distributed to another system. For maximum compatibility, it is recommended
> to use a library that sets styling at runtime, and detects if styling should be applied.
>
> You should only use this library if you understand what it means to apply styling at
> compile-time, and find that behavior preferable for your use.

# Example

```rust
use ansiform::ansi;

println!(ansi!("It {;green,bold}!"), "works");
println!(ansi!("Warning: {:#?;yellow}!"), Some("warning"));
```

# Features

This crate *should* support all format options defined in
[`std::fmt`](https://doc.rust-lang.org/std/fmt/). If it's missing something (and it probably will
before v1.0.0), submit an Issue or PR!

After defining formatting (like `:?` or `:#?`), you define modifiers with `;option`. You can add
multiple modifiers separated by `,`. So, to print debug format in underlined italic red,
`{:?;underline,red,italic}`.

## Options

###  Styles

- bold
- faint
- italic
- underline
- inverse/inverted

### Foreground Colors

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white
- bright black
- gray
- grey
- bright red
- bright green
- bright yellow
- bright blue
- bright magenta
- bright cyan
- bright white

### Background Colors

- on black
- on red
- on green
- on yellow
- on blue
- on magenta
- on cyan
- on white
- on bright black
- on gray
- on grey
- on bright red
- on bright green
- on bright yellow
- on bright blue
- on bright magenta
- on bright cyan
- on bright white
