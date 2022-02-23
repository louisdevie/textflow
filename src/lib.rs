#![deny(missing_docs)]

//! A library built upon [`textwrap`](https://crates.io/crates/textwrap)
//! that lets you print text in columns, aligned to the right, centered
//! or justified.

extern crate lazy_static;
extern crate regex;
extern crate textwrap;
extern crate unicode_width;

#[cfg(feature = "styling")]
extern crate perroquet;

/// (re-exported from `textwrap`) Holds settings for wrapping and filling text.
pub use textwrap::Options;

mod utils;

#[cfg(not(feature = "styling"))]
mod align;
#[cfg(feature = "styling")]
mod align_rich;

#[cfg(not(feature = "styling"))]
mod columns;
//#[cfg(feature = "styling")]
//mod columns_rich;

mod layout;

#[cfg(not(feature = "styling"))]
pub use align::align;
#[cfg(feature = "styling")]
pub use align_rich::align;

#[cfg(not(feature = "styling"))]
pub use columns::columns;
//#[cfg(feature = "styling")]
//pub use columns_rich::columns;

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
