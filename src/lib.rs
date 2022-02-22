#![deny(missing_docs)]

//! A library built upon [`textwrap`](https://crates.io/crates/textwrap)
//! that lets you print text in columns, aligned to the right, centered
//! or justified.

extern crate lazy_static;
extern crate regex;
extern crate textwrap;
extern crate unicode_width;

/// (re-exported from `textwrap`) Holds settings for wrapping and filling text.
pub use textwrap::Options;

mod content;
mod utils;

mod align;
mod columns;
mod layout;

pub use align::align;
pub use columns::columns;
pub use layout::Layout;

/// Text alignment. See [align()] for details.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    /// Left-aligned
    LEFT,
    /// Centered
    CENTER,
    /// Right-aligned
    RIGHT,
    /// Justified
    JUSTIFY,
}

/// Column spacing. See [columns()] for details.
#[derive(Copy, Clone, Debug)]
pub enum Spacing {
    /// No spacing
    NONE,
    /// Space between columns
    BETWEEN,
    /// Space between and around columns
    AROUND,
}
