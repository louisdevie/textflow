use crate::utils::*;

use crate::Alignment;
use lazy_static::lazy_static;
use regex::Regex;

/// Dynamic columns layout.
///
/// Layouts can support a variable number of columns
/// (see [`Layout::repeat`]) and have columns with dynamic
/// size (see [`Layout::fractioanl`]).
pub struct Layout {
    // columns before the repeating one
    left: Vec<DynCol>,
    // the repeating column
    fill: Option<DynCol>,
    // columns after the repeating one
    right: Vec<DynCol>,
}

// calculated layout column
#[derive(Debug, PartialEq)]
pub struct Column {
    pub width: usize,
    pub alignment: Alignment,
}

#[derive(Debug, PartialEq)]
struct DynCol {
    size: usize,
    unit: ColWidthUnit,
    align: Alignment,
}

#[derive(Debug, PartialEq)]
enum ColWidthUnit {
    CHARACTER,
    FRACTIONAL,
}

impl Layout {
    /// Creates a new blank layout.
    ///
    /// You need to add columns to the resulting layout
    /// with [Layout::fractional] or [Layout::fixed] before you can use it.
    ///
    /// # Example
    ///
    /// ```
    /// let my_layout = Layout::new()
    ///     .fixed(4, RIGHT)
    ///     .fractional(1, CENTER)
    ///     .repeat();
    /// ```
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            fill: None,
            right: Vec::new(),
        }
    }

    /// The default layout.
    ///
    /// Accept any number of columns, each one with a width
    /// of 1 fractional unit and aligned to the left.
    ///
    /// Equivalent to the pattern `<-*`.
    pub fn default() -> Self {
        Self::new().fractional(1, crate::Alignment::LEFT).repeat()
    }

    /// Creates a layout from a pattern.
    ///
    /// Returns the successfully built layout or an error message.
    ///
    /// # Pattern Syntax
    ///
    /// Patterns describes columns separated by spaces.
    ///
    /// Each column can be composed of :
    /// * An alignment :
    ///   * `<` for `LEFT`,
    ///   * `^` for `CENTER`,
    ///   * `>` for `RIGHT` or
    ///   * `=` for `JUSTIFY`
    ///
    ///   (see [align()](crate::align::align) for more information).
    ///   If it is not specified, it will default to `LEFT`.
    /// * A width :
    ///   * either an integer meaning a fixed width in characters
    ///     (see [Layout::fixed()] for more information);
    ///   * or one or more hyphens `-` meaning a dynamic width in fractional
    ///     units, with the number of hyphens being the width
    ///     (see [Layout::fractional()] for more information).
    ///
    ///   If it is not specified, it will default to 1 fractional unit.
    /// * A repeating flag: `*`, wich can only be used on one column
    ///   (see [Layout::repeat()] for more information).
    ///
    /// # Example
    ///
    /// ```
    /// Layout::from_pattern("^5 <-*").unwrap()
    /// ```
    /// can be read as “one column five characters wide followed by
    /// zero, one or more columns with equal width”
    /// and is equivalent to
    /// ```
    /// Layout::new()
    ///     .fixed(5, Alignment::CENTER)
    ///     .fractional(1, Alignment::LEFT).repeat()
    /// ```
    pub fn from_pattern(pattern: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE_COLUMN: Regex = Regex::new(r"^([<^>=]?)(-*|\d*)(\*?)$").unwrap();
            static ref RE_HYPHENS: Regex = Regex::new(r"^-+$").unwrap();
        }

        let mut parsed = Self::new();
        let mut already_repeated = false;

        for column_pattern in pattern.split(" ") {
            match RE_COLUMN.captures(column_pattern) {
                None => {
                    return Err(format!(
                        "Invalid column [{}] in pattern [{}]",
                        column_pattern, pattern
                    ))
                }

                Some(groups) => {
                    if groups.get(0).unwrap().as_str().is_empty() {
                        // skip multiple spaces
                        continue;
                    }

                    // alignment
                    let align = match groups.get(1).unwrap().as_str() {
                        "^" => crate::Alignment::CENTER,
                        ">" => crate::Alignment::RIGHT,
                        "=" => crate::Alignment::JUSTIFY,
                        // default alignment is left
                        &_ => crate::Alignment::LEFT,
                    };

                    // width
                    let size = groups.get(2).unwrap().as_str();
                    if size.is_empty() {
                        // default column size is 1fr
                        parsed = parsed.fractional(1, align);
                    } else if RE_HYPHENS.is_match(size) {
                        parsed = parsed.fractional(size.len(), align);
                    } else {
                        parsed = parsed.fixed(size.parse::<usize>().unwrap(), align);
                    }

                    // repeat
                    if groups.get(3).unwrap().as_str() == "*" {
                        if already_repeated {
                            return Err(String::from("Only one column can be repeated"));
                        } else {
                            parsed = parsed.repeat();
                            already_repeated = true;
                        }
                    }
                }
            }
        }
        return Ok(parsed);
    }

    /// Add a column with fractional size.
    ///
    /// The calculated width will be the width of
    /// this column divided by the sum of the widths
    /// of all the fractional columns (similar
    /// to the `fr` unit in css).
    pub fn fractional(mut self, size: usize, alignment: Alignment) -> Self {
        match self.fill {
            None => self.left.push(DynCol {
                unit: ColWidthUnit::FRACTIONAL,
                size,
                align: alignment,
            }),
            Some(_) => self.right.push(DynCol {
                unit: ColWidthUnit::FRACTIONAL,
                size,
                align: alignment,
            }),
        }
        return self;
    }

    /// Add a column with a fixed width in characters.
    ///
    /// The calculated width will be exactly the same.
    pub fn fixed(mut self, size: usize, alignment: Alignment) -> Self {
        match self.fill {
            None => self.left.push(DynCol {
                unit: ColWidthUnit::CHARACTER,
                size,
                align: alignment,
            }),
            Some(_) => self.right.push(DynCol {
                unit: ColWidthUnit::CHARACTER,
                size,
                align: alignment,
            }),
        }
        return self;
    }

    /// Set the last column of the layout as repeating
    ///
    /// The repeating column can be removed or
    /// duplicated any number of times to fit the
    /// requested number of columns (it can be compared
    /// to the `*` repetition in regular expressions)
    ///
    /// Only one column can be repeated. **This function
    /// will panic if it is called on a layout that
    /// already has a repeating column**.
    /// A `try_repeat` function may be added in a future
    /// version if needed.
    pub fn repeat(mut self) -> Self {
        match self.fill {
            None => self.fill = Some(self.left.pop().expect("No column to repeat")),
            Some(_) => panic!("Only one column can be repeated"),
        }
        return self;
    }

    /// Calculate all the dynamic columns for a given width
    /// and number of columns.
    pub fn resolve(&self, width: usize, columns: usize) -> Result<Vec<Column>, String> {
        let missing = columns as i32 - self.left.len() as i32 - self.right.len() as i32;

        if missing < 0 {
            return Err(format!(
                "Expected at least {} columns, got {}",
                self.left.len() + self.right.len(),
                columns
            ));
        } else {
            let mut expanded = Vec::new();

            for col in &self.left {
                expanded.push(col);
            }
            match &self.fill {
                None => {
                    if missing > 0 {
                        return Err(format!(
                            "Expected {} columns, got {}",
                            self.left.len(),
                            columns
                        ));
                    }
                }
                Some(col) => {
                    for _ in 0..missing {
                        expanded.push(col);
                    }
                    for col in &self.right {
                        expanded.push(col);
                    }
                }
            }

            let fractions: usize = expanded
                .iter()
                .filter(|col| col.unit == ColWidthUnit::FRACTIONAL)
                .map(|col| col.size)
                .sum();
            let absolute: usize = expanded
                .iter()
                .filter(|col| col.unit == ColWidthUnit::CHARACTER)
                .map(|col| col.size)
                .sum();
            let remaining = std::cmp::max(fractions as i32, width as i32 - absolute as i32)
                .try_into()
                .unwrap();
            let mut resolved = Vec::new();

            let mut fractional_columns = split_evenly(remaining, fractions);
            for col in expanded {
                match col.unit {
                    ColWidthUnit::CHARACTER => resolved.push(Column {
                        width: col.size,
                        alignment: col.align,
                    }),
                    ColWidthUnit::FRACTIONAL => {
                        let mut total_width = 0;
                        for _ in 0..col.size {
                            total_width += fractional_columns.remove(0);
                        }
                        resolved.push(Column {
                            width: total_width,
                            alignment: col.align,
                        });
                    }
                }
            }

            return Ok(resolved);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let blank = Layout::new();

        assert_eq!(blank.left, vec![]);
        assert_eq!(blank.fill, None);
        assert_eq!(blank.right, vec![]);
    }

    #[test]
    fn test_default() {
        let default = Layout::default();

        assert_eq!(default.left, vec![]);
        assert_eq!(
            default.fill,
            Some(DynCol {
                size: 1,
                unit: ColWidthUnit::FRACTIONAL,
                align: Alignment::LEFT,
            })
        );
        assert_eq!(default.right, vec![]);
    }

    #[test]
    fn test_from_pattern() {
        let valid1 = Layout::from_pattern("^7 - 3* =").unwrap();

        assert_eq!(
            valid1.left,
            vec![
                DynCol {
                    size: 7,
                    unit: ColWidthUnit::CHARACTER,
                    align: Alignment::CENTER,
                },
                DynCol {
                    size: 1,
                    unit: ColWidthUnit::FRACTIONAL,
                    align: Alignment::LEFT,
                }
            ]
        );
        assert_eq!(
            valid1.fill,
            Some(DynCol {
                size: 3,
                unit: ColWidthUnit::CHARACTER,
                align: Alignment::LEFT,
            })
        );
        assert_eq!(
            valid1.right,
            vec![DynCol {
                size: 1,
                unit: ColWidthUnit::FRACTIONAL,
                align: Alignment::JUSTIFY,
            }]
        );

        let valid2 = Layout::from_pattern(">-  <--  =---").unwrap();

        assert_eq!(
            valid2.left,
            vec![
                DynCol {
                    size: 1,
                    unit: ColWidthUnit::FRACTIONAL,
                    align: Alignment::RIGHT,
                },
                DynCol {
                    size: 2,
                    unit: ColWidthUnit::FRACTIONAL,
                    align: Alignment::LEFT,
                },
                DynCol {
                    size: 3,
                    unit: ColWidthUnit::FRACTIONAL,
                    align: Alignment::JUSTIFY,
                }
            ]
        );
        assert_eq!(valid2.fill, None);
        assert_eq!(valid2.right, vec![]);

        let invalid1 = Layout::from_pattern("^7 +8 9");

        assert!(invalid1.is_err());

        let invalid2 = Layout::from_pattern("^7 8* 9* 10");

        assert!(invalid2.is_err());
    }

    #[test]
    fn test_fractional() {
        let mock = Layout::new().fractional(4, Alignment::CENTER);

        assert_eq!(
            mock.left,
            vec![DynCol {
                size: 4,
                unit: ColWidthUnit::FRACTIONAL,
                align: Alignment::CENTER,
            }]
        );
        assert_eq!(mock.fill, None);
        assert_eq!(mock.right, vec![]);

        let muck = mock.repeat().fractional(9, Alignment::RIGHT);

        assert_eq!(muck.left, vec![]);
        assert_eq!(
            muck.fill,
            Some(DynCol {
                size: 4,
                unit: ColWidthUnit::FRACTIONAL,
                align: Alignment::CENTER,
            })
        );
        assert_eq!(
            muck.right,
            vec![DynCol {
                size: 9,
                unit: ColWidthUnit::FRACTIONAL,
                align: Alignment::RIGHT,
            }]
        );
    }

    #[test]
    fn test_fixed() {
        let mock = Layout::new().fixed(4, Alignment::CENTER);

        assert_eq!(
            mock.left,
            vec![DynCol {
                size: 4,
                unit: ColWidthUnit::CHARACTER,
                align: Alignment::CENTER,
            }]
        );
        assert_eq!(mock.fill, None);
        assert_eq!(mock.right, vec![]);

        let muck = mock.repeat().fixed(9, Alignment::RIGHT);

        assert_eq!(muck.left, vec![]);
        assert_eq!(
            muck.fill,
            Some(DynCol {
                size: 4,
                unit: ColWidthUnit::CHARACTER,
                align: Alignment::CENTER,
            })
        );
        assert_eq!(
            muck.right,
            vec![DynCol {
                size: 9,
                unit: ColWidthUnit::CHARACTER,
                align: Alignment::RIGHT,
            }]
        );
    }

    #[test]
    fn test_repeat_pass() {
        let valid = Layout::new().fixed(4, Alignment::CENTER).repeat();

        assert_eq!(valid.left, vec![]);
        assert_eq!(
            valid.fill,
            Some(DynCol {
                size: 4,
                unit: ColWidthUnit::CHARACTER,
                align: Alignment::CENTER,
            })
        );
        assert_eq!(valid.right, vec![]);
    }
    #[test]
    #[should_panic(expected = "Only one column can be repeated")]
    fn test_repeat_fail() {
        let _invalid = Layout::new()
            .fixed(4, Alignment::CENTER)
            .repeat()
            .fractional(1, Alignment::LEFT)
            .repeat();
    }

    #[test]
    fn test_resolve() {
        let generic = Layout::from_pattern(">5 -*").unwrap();

        // common use case
        assert_eq!(
            generic.resolve(23, 4),
            Ok(vec![
                Column {
                    width: 5,
                    alignment: Alignment::RIGHT
                },
                Column {
                    width: 6,
                    alignment: Alignment::LEFT
                },
                Column {
                    width: 6,
                    alignment: Alignment::LEFT
                },
                Column {
                    width: 6,
                    alignment: Alignment::LEFT
                }
            ])
        );
        // not enough space
        assert_eq!(
            generic.resolve(3, 3),
            Ok(vec![
                Column {
                    width: 5,
                    alignment: Alignment::RIGHT
                },
                Column {
                    width: 1,
                    alignment: Alignment::LEFT
                },
                Column {
                    width: 1,
                    alignment: Alignment::LEFT
                },
            ])
        );
        // not enough columns
        assert!(generic.resolve(23, 0).is_err());

        let fixed_only = Layout::from_pattern(">5 10* 15").unwrap();

        // will always be the same size
        assert_eq!(
            fixed_only.resolve(11, 3),
            Ok(vec![
                Column {
                    width: 5,
                    alignment: Alignment::RIGHT
                },
                Column {
                    width: 10,
                    alignment: Alignment::LEFT
                },
                Column {
                    width: 15,
                    alignment: Alignment::LEFT
                }
            ])
        );
        assert_eq!(
            fixed_only.resolve(44, 3),
            Ok(vec![
                Column {
                    width: 5,
                    alignment: Alignment::RIGHT
                },
                Column {
                    width: 10,
                    alignment: Alignment::LEFT
                },
                Column {
                    width: 15,
                    alignment: Alignment::LEFT
                }
            ])
        );

        let no_repeat = Layout::from_pattern("11 -- 22").unwrap();

        // not enough columns
        assert!(no_repeat.resolve(99, 2).is_err());
        // too many columns
        assert!(no_repeat.resolve(99, 5).is_err());
    }
}
