//! # colorline
//!
//! Minimal text coloring in Rust for terminals.
//!
//! `colorline` provides a simple trait, [`Style`], that adds methods
//! like `.red()`, `.blue()`, or `.bold()` directly to strings. It is
//! designed to stay minimal and easy to use, without extra features or
//! complexity.
//!
//! ## Supported styles
//! - Red
//! - Green
//! - Yellow
//! - Blue
//! - Magenta
//! - Cyan
//! - Bold
//!
//! ## Example
//! ```rust
//! use colorline::Style;
//!
//! fn main() {
//!     println!("{}", "Error".red().bold());
//!     println!("{}", "Success".green());
//!     println!("{}", "Note".cyan());
//! }
//! ```
//!
pub trait Style {
    /// Colors the text red.
    fn red(&self) -> String;

    /// Colors the text green.
    fn green(&self) -> String;

    /// Colors the text yellow.
    fn yellow(&self) -> String;

    /// Colors the text blue.
    fn blue(&self) -> String;

    /// Colors the text magenta (purple).
    fn magenta(&self) -> String;

    /// Colors the text cyan (light blue).
    fn cyan(&self) -> String;
    
    /// Makes the text bold.
    fn bold(&self) -> String;
}

/// Implementation of `Style` for string slices (`&str`).
/// 
/// This allows you to call methods like `"Hello".red()` directly.
impl Style for &str {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }
    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }
    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }
    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self)
    }
    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }
}

/// Implementation of `Style` for owned `String`.
/// 
/// This delegates to the `&str` implementation, so you can also call
/// methods like `my_string.red()` if you have a `String`.
impl Style for String {
    fn red(&self) -> String { self.as_str().red() }
    fn green(&self) -> String { self.as_str().green() }
    fn yellow(&self) -> String { self.as_str().yellow() }
    fn blue(&self) -> String { self.as_str().blue() }
    fn magenta(&self) -> String { self.as_str().magenta() }
    fn cyan(&self) -> String { self.as_str().cyan() }
    fn bold(&self) -> String { self.as_str().bold() }
}
