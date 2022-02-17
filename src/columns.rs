use crate::utils::*;

use crate::Layout;
use crate::Spacing;

/// Wraps and aligns text within columns.
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
    layout: Layout,
    width_or_options: TextwrapOptions,
) -> String
where
    StringsCollection: IntoIterator<Item = &'a str>,
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm + Clone,
    TextwrapWordSep: textwrap::word_separators::WordSeparator + Clone,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter,
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
