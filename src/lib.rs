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

/// Text alignment. See [align()] for details.
#[derive(Copy, Clone)]
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
