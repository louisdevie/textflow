use crate::utils::*;

use crate::Alignment;
use unicode_width::UnicodeWidthStr as UniW;

/// Wraps and aligns text.
///
/// `width_or_options` can either be an integer or [textwrap::Options],
///  see the documentation of [`textwrap`]() for more information.
///
/// There are four alignment modes :
/// * `LEFT` doesn't modifiy the text, so it ends up left-aligned
/// * `RIGHT` pad each line with spaces so that the text is right-aligned
///           to the width specified by `width_or_options`
/// * `CENTER` pad each line with spaces so that the text is centered inside
///            the width specified by `width_or_options`
/// * `JUSTIFY` expand spaces so that each line (except the last one) are
///             filling the width specified by `width_or_options`
///
/// # Examples
///
/// ```
/// use textflow::align;
/// use textflow::Alignment::*;
///
/// fn main() {
///     let text = "textflow:\na small extension for textwrap.";
///     println!("{}", align(text, CENTER, 20));
/// }
/// ```
/// should display
/// ```text
///      textflow:
///  a small extension
///    for textwrap.
/// ```
///
/// The crate also contains [an example](https://github.com/louisdevie/textflow/blob/main/examples/alignment.rs).
pub fn align<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit, TextwrapOptions>(
    text: &str,
    alignment: Alignment,
    width_or_options: TextwrapOptions,
) -> String
where
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm,
    TextwrapWordSep: textwrap::word_separators::WordSeparator,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter,
    TextwrapOptions: Into<textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>>,
{
    let options = width_or_options.into();

    // copy the width before passing the options
    // to `wrap` because it consumes it
    let width = options.width;

    let wrapped = textwrap::wrap(text, options);

    let mut wrapped_and_aligned = String::new();

    for (i, line) in wrapped.iter().enumerate() {
        let last_line = i == wrapped.len() - 1;
        wrapped_and_aligned.push_str(&align_line(line, width, alignment, last_line));
        // no line feed at the end
        if !last_line {
            wrapped_and_aligned.push('\n');
        }
    }

    return wrapped_and_aligned;
}

// real deal
pub fn align_line(line: &str, width: usize, alignment: Alignment, last: bool) -> String {
    let remaining = width - UniW::width(line);

    match alignment {
        // pad at the end (useful for `columns`)
        Alignment::LEFT => String::from(line) + &" ".repeat(remaining),

        // pad at the start
        Alignment::RIGHT => " ".repeat(remaining) + line,

        // pad each side (again, the padding at the end is useful for `columns`)
        Alignment::CENTER => {
            let before = remaining / 2;
            " ".repeat(before) + line + &" ".repeat(remaining - before)
        }

        // now onto the complicated stuff
        Alignment::JUSTIFY => {
            if last {
                // the last line doesn't get justified
                String::from(line) + &" ".repeat(remaining)
            } else {
                let mut words: Vec<&str> = line.split(" ").collect();
                // distribute spaces
                let spaces = split_evenly(words.len() + remaining - 1, words.len() - 1);

                // the first word is treated separately
                let mut aligned = if words.len() != 0 {
                    // `remove(0)` will panics if the vector is empty ...
                    String::from(words.remove(0))
                } else {
                    // ... it means the line is empty so we return an empty string
                    String::new()
                };
                if words.len() == 0 {
                    // only one word
                    aligned.push_str(&" ".repeat(remaining));
                } else {
                    for (word, spacing) in words.iter().zip(spaces) {
                        aligned.push_str(&" ".repeat(spacing));
                        aligned.push_str(word);
                    }
                }

                aligned
            }
        }
    }
}
