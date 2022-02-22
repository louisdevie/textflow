use crate::utils::*;

use crate::Layout;
use crate::Spacing;

/// Wraps and aligns text within columns.
///
/// `content` may be an array or a vector of strings.
/// The number of columns is the number of strings,
/// not based on the layout.
///
/// `width_or_options` can either be an integer or [textwrap::Options],
/// see the documentation of `textwrap` for more information.
/// (Note that the options `break_word` and `word_splitter` are overriden
/// to `true` and `textwrap::word_splitters::HyphenSplitter` respectively.)
///
/// There are three spacing modes :
/// * `BETWEEN` add margins bewteen the columns
/// * `AROUND` add margins between and around the columns
/// * `NONE` doesn't add any margins
///
/// # Examples
///
/// ```
/// use textflow::columns;
/// use textflow::Layout;
/// use textflow::Spacing;
///
/// fn main() {
///     let text = [
///         "I am aligned to the left.",
///         "I am aligned to the right and take two times more space.",
///     ];
///     let layout = Layout::from_pattern("<- >--").unwrap();
///     println!("{}", columns(text, Spacing::BETWEEN, &layout, 31));
/// }
/// ```
/// should display
/// ```
/// I am        I am aligned to the
/// aligned to   right and take two
/// the left.     times more space.
/// ```
///
/// [Another example](https://github.com/louisdevie/textflow/blob/main/examples/columns.rs)
/// is included in the crate.
pub fn columns<
    'a,
    StringsCollection,
    TextwrapAlgo,
    TextwrapWordSep,
    TextwrapWordSplit,
    TextwrapOptions,
>(
    content: StringsCollection,
    spacing: Spacing,
    layout: &Layout,
    width_or_options: TextwrapOptions,
) -> String
where
    StringsCollection: IntoIterator<Item = &'a str>,
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm + Clone,
    TextwrapWordSep: textwrap::word_separators::WordSeparator + Clone,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter + Clone,
    TextwrapOptions: Into<textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>>,
{
    let options = width_or_options.into();

    let vectorised: Vec<&str> = content.into_iter().collect();

    let columns = layout
        .resolve(
            options.width - spacing_needed(spacing, vectorised.len()),
            vectorised.len(),
        )
        .unwrap();

    let mut wrapped = Vec::new();
    for (content, column) in vectorised.iter().zip(columns.iter()) {
        wrapped.push(textwrap::wrap(
            content,
            copy_textwrap_options(&options, column.width),
        ));
    }
    // group lines together
    crate::utils::invert_2d_vec(&mut wrapped);

    let mut formatted = String::new();

    for (i, line) in wrapped.iter().enumerate() {
        let last_line = i == wrapped.len() - 1;
        for (i, (content, column)) in line.iter().zip(columns.iter()).enumerate() {
            match spacing {
                Spacing::NONE => {}
                Spacing::BETWEEN => {
                    if i != 0 {
                        formatted.push(' ');
                    }
                }
                Spacing::AROUND => {
                    formatted.push(' ');
                }
            }
            formatted.push_str(&crate::align::align_line(
                content,
                column.width,
                column.alignment,
                last_line,
            ));
        }
        // no line feed at the end
        if !last_line {
            formatted.push('\n');
        }
    }

    return formatted;
}

#[test]
fn test_columns() {
    // the doctest
    let text = [
        "I am aligned to the left.",
        "I am aligned to the right and take two times more space.",
    ];

    let layout = Layout::from_pattern("<- >--").unwrap();

    let expected_none = String::from("I am       I am aligned to the\naligned to  right and take two\nthe left.    times more space.");
    let expected_between = String::from("I am        I am aligned to the\naligned to   right and take two\nthe left.     times more space.");
    // note that `Spacing::AROUND` leaves the end of the line empty (because laziness)
    let expected_around = String::from(" I am        I am aligned to the\n aligned to   right and take two\n the left.     times more space.");

    assert_eq!(columns(text, Spacing::NONE, &layout, 30), expected_none);
    assert_eq!(
        columns(text, Spacing::BETWEEN, &layout, 31),
        expected_between
    );
    assert_eq!(columns(text, Spacing::AROUND, &layout, 33), expected_around);
}
