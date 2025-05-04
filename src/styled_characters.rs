use super::charview::screen_character::ScreenCharacter;
use tui::style::Style as TuiStyle;

pub use super::charview::{CharChunkMap, ViewportLocation};
pub use tui::style::{Color as GameColor, Modifier as Font};

pub use crossterm::event::KeyCode as GameEvent;

pub use super::{Message, SCREEN_HEIGHT, SCREEN_WIDTH};

/// This struct models how to show a character in Termgame.
///
/// To use it, you can do the following:
///
/// ```rust
/// use termgame::{GameStyle, GameColor, Font};
/// let style = GameStyle::new()
///                       .color(Some(GameColor::Blue))
///                       .background_color(Some(GameColor::Red))
///                       .font(Some(Font::BOLD | Font::UNDERLINED));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Style {
    /// The color of the text.
    pub color: Option<GameColor>,
    /// The color of the background.
    pub background_color: Option<GameColor>,
    /// See [`Font`] for details, it decides bold/italic/underline/etc.
    pub font: Option<Font>,
}

impl Style {
    /// Create a new style that doesn't do anything.
    pub fn new() -> Style {
        Style {
            color: None,
            background_color: None,
            font: None,
        }
    }

    /// Apply the [`GameColor`] specified as the foreground.
    pub fn color(mut self, color: Option<GameColor>) -> Style {
        self.color = color;
        self
    }

    /// Apply the [`GameColor`] specified as the background.
    pub fn background_color(mut self, background_color: Option<GameColor>) -> Style {
        self.background_color = background_color;
        self
    }

    /// Apply the [`Font`] specified.
    pub fn font(mut self, font: Option<Font>) -> Style {
        self.font = font;
        self
    }
}

/// A character with a given style.
/// ```rust
/// use termgame::{StyledCharacter, GameStyle, GameColor};
/// StyledCharacter::new('x')
///                  .style(GameStyle::new().background_color(Some(GameColor::Black)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct StyledCharacter {
    /// This is the actual character that will be displayed on screen.
    pub c: char,
    /// This is the [`Style`] that will be used when displaying on screen.
    pub style: Option<Style>,
}

impl StyledCharacter {
    /// Create a new [`StyledCharacter`], based on the given character.
    pub fn new(c: char) -> Self {
        StyledCharacter { c, style: None }
    }

    /// Change the character.
    pub fn character(mut self, c: char) -> Self {
        self.c = c;
        self
    }

    /// Set the style.
    pub fn style(mut self, s: Style) -> Self {
        self.style = Some(s);
        self
    }
}

impl From<char> for StyledCharacter {
    fn from(c: char) -> Self {
        StyledCharacter { c, style: None }
    }
}

impl From<StyledCharacter> for ScreenCharacter {
    fn from(styled_char: StyledCharacter) -> Self {
        match styled_char.style {
            Some(s) => ScreenCharacter {
                c: styled_char.c,
                style: Some(TuiStyle {
                    fg: s.color,
                    bg: s.background_color,
                    add_modifier: s.font.unwrap_or(Font::empty()),
                    sub_modifier: Font::empty(),
                }),
            },
            None => ScreenCharacter {
                c: styled_char.c,
                style: None,
            },
        }
    }
}

impl From<ScreenCharacter> for StyledCharacter {
    fn from(screen_char: ScreenCharacter) -> Self {
        match screen_char.style {
            Some(s) => StyledCharacter {
                c: screen_char.c,
                style: Some(Style {
                    color: s.fg,
                    background_color: s.bg,
                    font: Some(s.add_modifier).filter(|m| *m != Font::empty()),
                }),
            },
            None => StyledCharacter {
                c: screen_char.c,
                style: None,
            },
        }
    }
}
