#![deny(missing_docs)]

//! A library built upon [`textwrap`](https://crates.io/crates/textwrap)
//! that lets you print text in columns, aligned to the right, centered
//! or justified.

extern crate textwrap;

/// (re-exported from `textwrap`) Holds settings for wrapping and filling text.
pub use textwrap::Options;

mod align;
pub use align::align;

mod columns;
pub use columns::columns;

mod layout;
pub use layout::Layout;

mod utils;

/// Text alignment. See [align()] for details.
#[derive(Copy, Clone, Debug)]
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
