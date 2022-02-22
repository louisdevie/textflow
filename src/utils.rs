// behind the scene

// divides `number` into `into` integer parts the most evenly possible
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

#[test]
fn test_split_evenly() {
    // common use case
    assert_eq!(split_evenly(23, 5), vec![5, 4, 5, 4, 5]);

    // edge cases
    assert_eq!(split_evenly(3, 7), vec![0, 1, 0, 1, 0, 1, 0]);

    assert_eq!(split_evenly(10, 0), vec![]);
}

// creates new options for textwrap
pub fn copy_textwrap_options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>(
    original: &'a textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>,
    new_width: usize,
) -> textwrap::Options<'a, TextwrapAlgo, TextwrapWordSep, TextwrapWordSplit>
where
    TextwrapAlgo: textwrap::wrap_algorithms::WrapAlgorithm + Clone,
    TextwrapWordSep: textwrap::word_separators::WordSeparator + Clone,
    TextwrapWordSplit: textwrap::word_splitters::WordSplitter + Clone,
{
    textwrap::Options::new(new_width)
        .initial_indent(original.initial_indent)
        .subsequent_indent(original.subsequent_indent)
        .word_separator(original.word_separator.clone())
        .wrap_algorithm(original.wrap_algorithm.clone())
        .word_splitter(original.word_splitter.clone())
}

#[test]
fn test_copy_textwrap_options() {
    let input = textwrap::Options::new(80)
        .initial_indent("* ")
        .subsequent_indent("| ")
        .break_words(false)
        .word_separator(textwrap::word_separators::AsciiSpace)
        .word_splitter(textwrap::word_splitters::NoHyphenation);

    let expected_output = textwrap::Options::new(20)
        .initial_indent("* ")
        .subsequent_indent("| ")
        .break_words(true);

    // test if types match (can't compile otherwise)
    let output: textwrap::Options<
        '_,
        textwrap::wrap_algorithms::OptimalFit,
        textwrap::word_separators::AsciiSpace,
        textwrap::word_splitters::NoHyphenation,
    > = copy_textwrap_options(&input, 20);

    assert_eq!(output.width, expected_output.width);
    assert_eq!(output.initial_indent, expected_output.initial_indent);
    assert_eq!(output.subsequent_indent, expected_output.subsequent_indent);
    assert_eq!(output.break_words, expected_output.break_words);
}

// invert a 2-dimensional vector
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
            swap_2d_vec(v, (i, j), (j, i));
        }
    }

    // truncate the vector
    if height > width {
        v.truncate(width)
    } else {
        for u in v.iter_mut() {
            u.truncate(height);
        }
    }
}

#[test]
fn test_invert_2d_vec() {
    // "horizontal" vector
    let mut vec1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
    invert_2d_vec(&mut vec1);
    assert_eq!(vec1, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);

    // "vertical" vector
    let mut vec2 = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    invert_2d_vec(&mut vec2);
    assert_eq!(vec2, vec![vec![1, 3, 5], vec![2, 4, 6]]);

    // "square" vector
    let mut vec3 = vec![vec![1, 2], vec![3, 4]];
    invert_2d_vec(&mut vec3);
    assert_eq!(vec3, vec![vec![1, 3], vec![2, 4]]);

    // empty vector
    let mut vec4: Vec<Vec<i8>> = vec![vec![], vec![], vec![]];
    invert_2d_vec(&mut vec4);
    assert_eq!(vec4, Vec::<Vec<i8>>::new());
}

// swap two values in a vector
fn swap_2d_vec<T>(v: &mut Vec<Vec<T>>, pos1: (usize, usize), pos2: (usize, usize))
where
    T: Default,
{
    let mut temp = T::default();
    std::mem::swap(&mut v[pos1.0][pos1.1], &mut temp);
    std::mem::swap(&mut v[pos2.0][pos2.1], &mut temp);
    std::mem::swap(&mut v[pos1.0][pos1.1], &mut temp);
}

// space to allocate for spacing
pub fn spacing_needed(spacing: crate::Spacing, columns: usize) -> usize {
    match spacing {
        crate::Spacing::NONE => 0,
        crate::Spacing::BETWEEN => {
            if columns == 0 {
                0
            } else {
                columns - 1
            }
        }
        crate::Spacing::AROUND => columns + 1,
    }
}

#[test]
fn test_spacing_needed() {
    assert_eq!(spacing_needed(crate::Spacing::NONE, 5), 0);
    assert_eq!(spacing_needed(crate::Spacing::NONE, 46), 0);

    assert_eq!(spacing_needed(crate::Spacing::BETWEEN, 5), 4);
    assert_eq!(spacing_needed(crate::Spacing::BETWEEN, 46), 45);
    assert_eq!(spacing_needed(crate::Spacing::BETWEEN, 0), 0);

    assert_eq!(spacing_needed(crate::Spacing::AROUND, 5), 6);
    assert_eq!(spacing_needed(crate::Spacing::AROUND, 46), 47);
    assert_eq!(spacing_needed(crate::Spacing::AROUND, 0), 1);
}
