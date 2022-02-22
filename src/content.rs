pub struct Content<'lt> {
    string: &'lt str,
}

impl Content<'static> {
    #[cfg(feature = "console-style")]
    pub fn extract(self) -> (&str, console::Style) {
        (String::new(), console::Style::new())
    }

    #[cfg(not(feature = "console-style"))]
    pub fn extract(self) -> &'static str {
        self.string
    }
}

impl<'lt> From<&'lt str> for Content<'lt> {
    fn from(value: &'lt str) -> Self {
        Self { string: value }
    }
}

#[cfg(feature = "console-style")]
impl<'lt> From<crate::TextflowStyledObject<&'lt str>> for Content<'lt> {
	fn from(value: crate::TextflowStyledObject<&'lt str>) -> Self {
		Self {
			string: 
		}
	}
}