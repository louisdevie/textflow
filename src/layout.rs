extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

use crate::Alignment;

/// Dynamic columns layout.
pub struct Layout {
    left: Vec<Col>,
    fill: Option<Col>,
    right: Vec<Col>,
}

// calculated layout column
#[derive(Debug)]
pub struct Column {
    pub width: usize,
    pub alignment: Alignment,
}

impl Layout {
    /// Creates a new blank layout.
    ///
    /// You need to add columns to the resulting layout
    /// with [Layout::fractional] or [Layout::fixed] before you can use it.
    ///
    /// # Example
    ///
    /// TODO
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
    pub fn default() -> Self {
        Self::new().fractional(1, crate::Alignment::LEFT).repeat()
    }

    /// Creates a layout from a pattern.
    ///
    /// TODO
    ///
    /// # Pattern Syntax
    ///
    /// TODO
    ///
    /// # Examples
    ///
    /// TODO
    ///
    pub fn from_pattern(pattern: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE_COLUMN: Regex = Regex::new(r"^([<^>=]?)(-*|\d*)(\*?)$").unwrap();
            static ref RE_DASHES: Regex = Regex::new(r"^-+$").unwrap();
        }

        let mut parsed = Self::new();
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
                        // skips multiple spaces
                        continue;
                    }

                    let align = match groups.get(1).unwrap().as_str() {
                        "^" => crate::Alignment::CENTER,
                        ">" => crate::Alignment::RIGHT,
                        "=" => crate::Alignment::JUSTIFY,
                        // default alignment is left
                        &_ => crate::Alignment::LEFT,
                    };

                    let size = groups.get(2).unwrap().as_str();
                    if size.is_empty() {
                        // default column size is 1fr
                        parsed = parsed.fractional(1, align);
                    } else if RE_DASHES.is_match(size) {
                        parsed = parsed.fractional(size.len(), align);
                    } else {
                        parsed = parsed.fixed(size.parse::<usize>().unwrap(), align);
                    }

                    if groups.get(3).unwrap().as_str() == "*" {
                        parsed = parsed.repeat();
                    }
                }
            }
        }
        return Ok(parsed);
    }

    /// Add a fractional-sized column
    pub fn fractional(mut self, size: usize, alignment: Alignment) -> Self {
        match self.fill {
            None => self.left.push(Col {
                unit: ColWidthUnit::FRACTIONAL,
                size,
                align: alignment,
            }),
            Some(_) => self.right.push(Col {
                unit: ColWidthUnit::FRACTIONAL,
                size,
                align: alignment,
            }),
        }
        return self;
    }

    /// Add an fixed-width column
    pub fn fixed(mut self, size: usize, alignment: Alignment) -> Self {
        match self.fill {
            None => self.left.push(Col {
                unit: ColWidthUnit::CHARACTER,
                size,
                align: alignment,
            }),
            Some(_) => self.right.push(Col {
                unit: ColWidthUnit::CHARACTER,
                size,
                align: alignment,
            }),
        }
        return self;
    }

    /// Set the last column of the layout as repeating
    pub fn repeat(mut self) -> Self {
        match self.fill {
            None => self.fill = Some(self.left.pop().expect("No column to repeat")),
            Some(_) => panic!("Only one column can be repeated"),
        }
        return self;
    }

    /// Calculate all the dynamic columns for a given width
    /// and number of columns
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

            let fractions = expanded
                .iter()
                .filter(|col| col.unit == ColWidthUnit::FRACTIONAL)
                .map(|col| col.size)
                .sum();
            let absolute: usize = expanded
                .iter()
                .filter(|col| col.unit == ColWidthUnit::CHARACTER)
                .map(|col| col.size)
                .sum();
            let remaining = std::cmp::max(fractions, width - absolute);

            let mut resolved = Vec::new();

            let mut fractional_columns = crate::utils::split_evenly(remaining, fractions);
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

struct Col {
    size: usize,
    unit: ColWidthUnit,
    align: Alignment,
}

#[derive(PartialEq)]
enum ColWidthUnit {
    CHARACTER,
    FRACTIONAL,
}
