<h1 align="center">colorline</h1>
<p align="center">Text coloring in Rust for terminals</p>

## About

Colorline provides a simple trait for Rust strings, letting you call methods like `.red()` or `.blue()` directly.  
It is designed to stay minimal and easy to use.

Supported colors:
- Red
- Green
- Yellow
- Blue
- Magenta
- Cyan
- Grey

## Example

```rust
use colorline::Style;

fn main() {
    println!("{}", "Error".red());
    println!("{}", "Success".green());
    println!("{}", "Note".cyan());
}
```

## Why

There are already crates for coloring text in Rust, like `colored`, which is similar to `colorline`.  
Colorline exists because I wanted something smaller and simpler:  
- Just a few basic colors and bold text  
- No extra features or complexity  
- An API that feels clean and easy to use
