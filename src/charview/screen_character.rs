#![warn(missing_docs)]

use ratatui::style::Style;
use std::default::Default;

#[derive(Debug, Clone, Default, Copy)]
/// A `ScreenCharacter` is a character that will be displayed
/// on the screen.
///
/// A `ScreenCharacter` will always have a character (`c`), and may
/// optionally have a
/// [`Style`](https://docs.rs/tui/latest/tui/style/struct.Style.html), which sets properties.
pub struct ScreenCharacter {
    pub c: char,
    pub style: Option<Style>,
}

impl From<char> for ScreenCharacter {
    fn from(c: char) -> ScreenCharacter {
        ScreenCharacter { c, style: None }
    }
}
