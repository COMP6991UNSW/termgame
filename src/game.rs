#![warn(missing_docs)]
use super::charview::screen_character::ScreenCharacter;

pub use super::charview::{CharChunkMap, ViewportLocation};
pub use tui::style::{Color, Modifier as Font};

pub use crossterm::event::{
    Event as GameEvent, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseEvent,
};

pub use super::{Message, SCREEN_HEIGHT, SCREEN_WIDTH};

pub use crate::styled_characters::{Style as GameStyle, StyledCharacter};

/// This is an enum to make it easy to match on events.
#[derive(Debug, PartialOrd, Clone, PartialEq, Eq, Hash)]
pub enum SimpleEvent {
    /// This happens when the user holds Control
    WithControl(KeyCode),
    /// This happens when the user holds Alt
    WithAlt(KeyCode),
    /// This happens when the user holds Control AND Alt
    WithControlAlt(KeyCode),
    /// This happens when the user just presses a key
    Just(KeyCode),
    /// This is when an event is more complicated than a keypress.
    ComplexEvent(GameEvent),
}

impl From<SimpleEvent> for GameEvent {
    fn from(event: SimpleEvent) -> GameEvent {
        let (c, modifiers) = match event {
            SimpleEvent::WithControl(c) => (c, KeyModifiers::CONTROL),
            SimpleEvent::WithAlt(c) => (c, KeyModifiers::ALT),
            SimpleEvent::WithControlAlt(c) => (c, KeyModifiers::CONTROL | KeyModifiers::ALT),
            SimpleEvent::Just(c) => (c, KeyModifiers::NONE),
            SimpleEvent::ComplexEvent(e) => return e,
        };
        GameEvent::Key(KeyEvent::new(c, modifiers))
    }
}

impl From<GameEvent> for SimpleEvent {
    fn from(event: GameEvent) -> SimpleEvent {
        const CONTROL_ALT: KeyModifiers = KeyModifiers::CONTROL.union(KeyModifiers::ALT);
        match event {
            GameEvent::Key(KeyEvent {
                code, modifiers, ..
            }) => match modifiers.intersection(KeyModifiers::CONTROL | KeyModifiers::ALT) {
                CONTROL_ALT => SimpleEvent::WithControlAlt(code),
                KeyModifiers::CONTROL => SimpleEvent::WithControl(code),
                KeyModifiers::ALT => SimpleEvent::WithAlt(code),
                KeyModifiers::NONE => SimpleEvent::Just(code),
                _ => unreachable!(),
            },
            e => return SimpleEvent::ComplexEvent(e),
        }
    }
}

/// The Game struct is passed to all of the Controller's event methods,
/// to allow the implementor to view and modify the state of the game.
pub struct Game<'a> {
    /// This determines whether the game will end soon.
    pub(super) should_end: bool,
    /// If Some, a message will be shown at the bottom of the screen.
    pub(super) message: Option<Message>,
    /// The place in the viewport that is currently the top-left pixel.
    pub(super) viewport: ViewportLocation,
    /// The chunkmap of the display.
    pub(super) chunks: &'a mut CharChunkMap,
}

impl<'a> Game<'a> {
    /// Create a game, given a [`CharChunkMap`].
    pub fn new(chunks: &mut CharChunkMap) -> Game {
        Game {
            should_end: false,
            message: None,
            viewport: ViewportLocation { x: 0, y: 0 },
            chunks,
        }
    }

    /// Get the screen size, in the form of (x, (y1, y2))
    ///
    /// `x` is the width of the screen. `y1` is the height of
    /// the game area; and `y2` is the height of the question area.
    pub fn screen_size(&self) -> (u16, (u16, u16)) {
        match self.message {
            Some(ref m) => {
                let rows: u16 = (m.text.matches('\n').count() + 3).try_into().unwrap();
                (SCREEN_WIDTH, ((SCREEN_HEIGHT - rows), rows))
            }
            None => (SCREEN_WIDTH, (SCREEN_HEIGHT, 0)),
        }
    }

    /// Obtain the current message being shown.
    /// `None` if no message is showing.
    pub fn get_message(&self) -> &Option<Message> {
        &self.message
    }

    /// Set a new message to be shown; or if `message` is None,
    /// remove any current message.
    pub fn set_message(&mut self, message: Option<Message>) {
        self.message = message;
    }

    /// Returns `true` if the game is about to end. This
    /// is only `true` if `end_game` has been called.
    pub fn game_will_end(&self) -> bool {
        self.should_end
    }

    /// Ends the game. This event handler will never be called again.
    /// Other event handlers may be called at most once, before the
    /// game is ended and the `run_game` function ends.
    pub fn end_game(&mut self) {
        self.should_end = true;
    }

    /// Return the character at the given (x, y) coordinates.
    ///
    /// If the return is `None`, nothing is at those coordinates.
    /// If the return is `Some`, the [`StyledCharacter`] returned
    /// is the character at those coordinates..
    pub fn get_screen_char(&self, x: i32, y: i32) -> Option<StyledCharacter> {
        self.chunks.get(x, y).map(|x| StyledCharacter::from(*x))
    }

    /// Place the character at the given (x, y) coordinates.
    ///
    /// If `character` is `None`, remove anything at those coordinates.
    /// If `character` is `Some`, insert [`StyledCharacter`] at those coordinates.
    pub fn set_screen_char(&mut self, x: i32, y: i32, character: Option<StyledCharacter>) {
        match character {
            Some(c) => self.chunks.insert(x, y, ScreenCharacter::from(c)),
            None => {
                self.chunks.remove(x, y);
            }
        }
    }

    /// This function takes a mutable reference to a chunkmap and
    /// swaps it out for another one. This allows you to do things
    /// like keep multiple maps at once; or do efficient re-builds of
    /// the screen.
    ///
    /// ```rust
    /// use termgame::{CharChunkMap, Game};
    /// let mut chunkmap1 = CharChunkMap::new();
    /// let mut chunkmap2 = CharChunkMap::new();
    /// chunkmap2.insert(1, 1, 'a'.into());
    /// let mut game = Game::new(&mut chunkmap1);
    /// game.swap_chunkmap(&mut chunkmap2);
    /// ````
    pub fn swap_chunkmap(&mut self, chunkmap: &mut CharChunkMap) {
        std::mem::swap(self.chunks, chunkmap);
    }

    /// Get the current [`ViewportLocation`]. This tells you the
    /// top-left coordinate currently in view.
    pub fn get_viewport(&self) -> ViewportLocation {
        self.viewport
    }

    /// This sets the viewport (i.e. the top-left coordniate currently in view)
    /// to the provided [`ViewportLocation`].
    pub fn set_viewport(&mut self, viewport: ViewportLocation) {
        self.viewport = viewport;
    }
}
