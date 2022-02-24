use crate::utils::*;
use crate::Alignment;

use perroquet::style;
use perroquet::RichString;

use unicode_width::UnicodeWidthStr as UniW;

/// Wraps and aligns text.
///
/// `width_or_options` can either be an integer or [textwrap::Options],
///  see the documentation of `textwrap` for more information.
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
pub fn align<'a, Content, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit, TextwrapOptions>(
    text: Content,
    alignment: Alignment,
    width_or_options: TextwrapOptions,
) -> RichString
where
    Content: Into<RichString>,
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm,
    TextwrapWordSep: textwrap::word_separators::WordSeparator,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter,
    TextwrapOptions: Into<textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>>,
{
    let options = width_or_options.into();
    let text = text.into();

    // copy the width before passing the options
    // to `wrap` because it consumes it
    let width = options.width;

    let wrapped = textwrap::wrap(text.raw(), options);

    let mut wrapped_and_aligned = RichString::new();

    let mut cursor = 0;
    for (i, line) in wrapped.iter().enumerate() {
        let last_line = i == wrapped.len() - 1;

        wrapped_and_aligned.push(&align_line(
            text.substring(cursor, cursor + line.len()),
            width,
            alignment,
            last_line,
        ));

        cursor = cursor + line.len();

        // no line feed at the end
        if !last_line {
            wrapped_and_aligned.push_extend("\n");
        }
    }

    return wrapped_and_aligned;
}

// real deal
pub fn align_line(
    mut line: RichString,
    width: usize,
    alignment: Alignment,
    last: bool,
) -> RichString {
    let remaining = width - UniW::width(line.raw());

    match alignment {
        // pad at the end (useful for `columns`)
        Alignment::LEFT => {
            line.push_plain(&" ".repeat(remaining));
            return line;
        }

        // pad at the start
        Alignment::RIGHT => {
            line.insert_plain(0, &" ".repeat(remaining));
            return line;
        }

        // pad each side (again, the padding at the end is useful for `columns`)
        Alignment::CENTER => {
            let before = remaining / 2;
            line.insert_plain(0, &" ".repeat(before));
            line.push_plain(&" ".repeat(remaining - before));
            return line;
        }

        // now onto the complicated stuff
        Alignment::JUSTIFY => {
            if last {
                // the last line doesn't get justified
                line.push_plain(&" ".repeat(remaining));
            } else {
                let mut words: Vec<RichString> = line.split(" ");
                // distribute spaces
                let spaces = split_evenly(words.len() + remaining - 1, words.len() - 1);

                // the first word is treated separately
                line = words.remove(0);
                if words.len() == 0 {
                    // only one word
                    line.push_plain(&" ".repeat(remaining));
                } else {
                    for (word, spacing) in words.iter().zip(spaces) {
                        line.push_plain(&" ".repeat(spacing));
                        line.push(word);
                    }
                }
            }
            return line;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_line() {
        // left alignment
        assert_eq!(
            align_line(style!("even"), 10, Alignment::LEFT, false),
            style!("even      ")
        );

        // right alignment
        assert_eq!(
            align_line(style!("even"), 10, Alignment::RIGHT, false),
            style!("      even")
        );

        // center with an even number of characters left
        assert_eq!(
            align_line(style!("even"), 10, Alignment::CENTER, false),
            style!("   even   ")
        );
        // center with an odd number of characters left
        assert_eq!(
            align_line(style!("odd"), 10, Alignment::CENTER, false),
            style!("   odd    ")
        );

        // justified
        assert_eq!(
            align_line(style!("even odd odd even"), 19, Alignment::JUSTIFY, false),
            style!("even  odd odd  even")
        );
        // last line justified
        assert_eq!(
            align_line(style!("even odd odd even"), 19, Alignment::JUSTIFY, true),
            style!("even odd odd even  ")
        );
        // one word justified
        assert_eq!(
            align_line(style!("even"), 19, Alignment::JUSTIFY, false),
            style!("even               ")
        );
        // empty line justified
        assert_eq!(
            align_line(style!(""), 19, Alignment::JUSTIFY, false),
            style!("                   ")
        );

        // empty lines
        assert_eq!(
            align_line(style!(""), 10, Alignment::LEFT, false),
            style!("          ")
        );
        assert_eq!(
            align_line(style!(""), 10, Alignment::RIGHT, false),
            style!("          ")
        );
        assert_eq!(
            align_line(style!(""), 10, Alignment::CENTER, false),
            style!("          ")
        );
        assert_eq!(
            align_line(style!(""), 10, Alignment::JUSTIFY, false),
            style!("          ")
        );
        assert_eq!(
            align_line(style!(""), 10, Alignment::JUSTIFY, true),
            style!("          ")
        );
    }

    #[test]
    fn test_align() {
        // the doctest
        let text = "textflow:\na small extension for textwrap.";

        let expected = style!("     textflow:      \n a small extension  \n   for textwrap.    ");

        assert_eq!(align(text, Alignment::CENTER, 20), expected);
    }
}
