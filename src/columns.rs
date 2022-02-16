use crate::Layout;
use crate::Spacing;

/// Wraps and align text withi columns.
pub fn columns<'a, Strings>(
    content: Strings,
    _spacing: Spacing,
    layout: Layout,
    width: usize,
) -> String
where
    Strings: IntoIterator<Item = &'a str>,
{
    let vectorised: Vec<&str> = content.into_iter().collect();

    format!("{:#?}", layout.resolve(width, vectorised.len()))
}
