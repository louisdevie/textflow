extern crate textwrap;

/// Wraps and align the given `text`.
///
/// `width_or_options` can either be an integer or [textwrap::Options],
///  see `textwrap`'s documentation for more information.
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
    alignment: super::Alignment,
    width_or_options: TextwrapOptions,
) -> String
where
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm,
    TextwrapWordSep: textwrap::word_separators::WordSeparator,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter,
    TextwrapOptions: Into<textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>>,
{
    let options = width_or_options.into();

    // copy the width before passing the options to `wrap`
    let width = options.width;

    let wrapped = textwrap::wrap(text, options);

    let mut wrapped_and_aligned = String::new();

    for (i, line) in wrapped.iter().enumerate() {
        if i != 0 {
            wrapped_and_aligned.push('\n');
        }
        wrapped_and_aligned.push_str(&align_line(line, width, alignment, i == wrapped.len() - 1));
    }

    return wrapped_and_aligned;
}

fn align_line(line: &str, width: usize, alignment: super::Alignment, last: bool) -> String {
    let remaining = width - line.len();

    match alignment {
        super::Alignment::LEFT => String::from(line),

        super::Alignment::RIGHT => " ".repeat(remaining) + line,

        super::Alignment::CENTER => " ".repeat(remaining / 2) + line,

        super::Alignment::JUSTIFY => {
            if !last {
                let mut words: Vec<&str> = line.split(" ").collect();
                let spaces = split_evenly(words.len() + remaining - 1, words.len() - 1);

                // `remove(0)` panics if the vector is empty
                let mut aligned = if words.len() != 0 {
                    String::from(words.remove(0))
                } else {
                    String::new()
                };
                for (word, spacing) in words.iter().zip(spaces) {
                    aligned.push_str(&" ".repeat(spacing));
                    aligned.push_str(word);
                }

                aligned
            } else {
                String::from(line)
            }
        }
    }
}

fn split_evenly(number: usize, into: usize) -> Vec<usize> {
    let mut steps: Vec<usize> = Vec::new();
    for i in 0..(into + 1) {
        steps.push((i as f32 * (number as f32 / into as f32)).round() as usize);
    }

    let mut deltas = Vec::new();
    for i in 0..into {
        deltas.push(steps[i + 1] - steps[i]);
    }

    return deltas;
}
