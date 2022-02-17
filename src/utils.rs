//! DOC
//!
//! RM: THIS CRATE SHOULD NOT BE PUBLIC

// divides `number` into `into` integer parts the most evenly possible
/// DOC
pub fn split_evenly(number: usize, into: usize) -> Vec<usize> {
    // steps from 0 to `number`, separated by `number`/`into`
    // rounded to the nearest integer
    let mut steps: Vec<usize> = Vec::new();
    for i in 0..(into + 1) {
        steps.push((i as f32 * (number as f32 / into as f32)).round() as usize);
    }

    // what we want is the difference between each step
    let mut deltas = Vec::new();
    for i in 0..into {
        deltas.push(steps[i + 1] - steps[i]);
    }

    return deltas;
}

/// DOC
pub fn copy_textwrap_options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>(
    original: &'a textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>,
    new_width: usize,
) -> textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, textwrap::word_splitters::HyphenSplitter>
where
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm + Clone,
    TextwrapWordSep: textwrap::word_separators::WordSeparator + Clone,
{
    textwrap::Options::new(new_width)
        .initial_indent(original.initial_indent)
        .subsequent_indent(original.subsequent_indent)
        .word_separator(original.word_separator.clone())
        .wrap_algorithm(original.wrap_algorithm.clone())
}

/// DOC
pub fn invert_2d_vec<T>(v: &mut Vec<Vec<T>>)
where
    Vec<T>: Clone,
    T: Default + Clone,
{
    let height = v.len();
    let width = v.iter().map(|v| v.len()).max().unwrap();

    // expand the vector into a square
    let size = if height > width {
        for u in v.iter_mut() {
            u.resize_with(height, Default::default);
        }
        height
    } else {
        for u in v.iter_mut() {
            u.resize_with(width, Default::default);
        }
        v.resize(
            width,
            std::iter::repeat_with(Default::default)
                .take(width)
                .collect(),
        );
        width
    };

    // swap the values
    for i in 0..(size - 1) {
        for j in (1 + i)..(size) {
            swap_2d_vec(&mut v, (i, j), (j, i));
        }
    }
}

fn swap_2d_vec<T>(v: &mut Vec<Vec<T>>, pos1: (usize, usize), pos2: (usize, usize))
where
    T: Default,
{
    let mut temp = T::default();
}
