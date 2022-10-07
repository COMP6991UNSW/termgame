#![warn(missing_docs)]
//! TermGame is a small crate that extends tui-rs to make it easy
//! to write TUI games.
//!
//! Mainly used in COMP6991 at the University of New South Wales,
//! the crate provides the [`Controller`] trait, which is accepted
//! by the [`run_game`] function to start a game using a Crossterm
//! TUI (provided by tui-rs).
//!
//! It also wraps many tui features, like [`StyledCharacter`],
//! [`GameEvent`], and [`GameStyle`]
//!
//! ```no_run
//!
//! use termgame::{SimpleEvent, Controller, Game, GameEvent, GameSettings, StyledCharacter, run_game, KeyCode};
//! use std::error::Error;
//! use std::time::Duration;
//!
//! struct MyGame {}
//!
//! impl Controller for MyGame {
//!     fn on_start(&mut self, game: &mut Game) {
//!     }
//!
//!     fn on_event(&mut self, game: &mut Game, event: GameEvent) {
//!         match event.into() {
//!             SimpleEvent::Just(KeyCode::Char(ch)) => {
//!                 game.set_screen_char(1, 1, Some(StyledCharacter::new(ch)))
//!             },
//!             _ => {}
//!         }
//!
//!     }
//!
//!     fn on_tick(&mut self, _game: &mut Game) {}
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut controller = MyGame {};
//!
//!     run_game(
//!         &mut controller,
//!         GameSettings::new()
//!             // The below are the defaults, but shown so you can edit them.
//!             .tick_duration(Duration::from_millis(50))
//!             .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into()))
//!     )?;
//!
//!     println!("Game Ended!");
//!
//!     Ok(())
//! }
//! ```

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

mod charview;
mod controller;
mod game;
mod game_error;
mod message;
mod styled_characters;

pub use controller::Controller;
pub use game::{
    Color as GameColor, Game, GameEvent, GameStyle, KeyCode, KeyEvent, KeyEventKind, KeyEventState,
    KeyModifiers, MouseEvent, SimpleEvent, StyledCharacter, ViewportLocation,
};
pub use game_error::GameError;
pub use message::Message;
pub use tui::style::Modifier as Font;

pub use charview::{chunkmap::ChunkMap, CharChunkMap, CharView};

/// The required screen height termgame can play at.
/// Set to the size of a standard vt100
pub const SCREEN_HEIGHT: u16 = 24;
/// The required screen width termgame can play at.
pub const SCREEN_WIDTH: u16 = 80;

/// This struct allows you to configure how [`run_game`] works.
#[derive(Debug, Clone)]
pub struct GameSettings {
    /// This specfies how often your game will re-render.
    /// Shorter duration lead to more responsive games, but too high may
    /// lead to lag in your game.
    tick_duration: Duration,

    /// This specifies what key combination will cause the game to end.
    /// By default this is Ctrl-C
    quit_event: Option<Event>,
}

impl GameSettings {
    /// Creates a default GameSettings struct.
    pub fn new() -> GameSettings {
        GameSettings::default()
    }

    /// Set the tick_duration.
    pub fn tick_duration(mut self, tick_duration: Duration) -> GameSettings {
        self.tick_duration = tick_duration;
        self
    }

    /// Set a new key combination to quit the game (or disable it entirely).
    pub fn quit_event(mut self, quit_event: Option<Event>) -> GameSettings {
        self.quit_event = quit_event;
        self
    }
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            tick_duration: Duration::from_millis(50),
            quit_event: Some(SimpleEvent::WithControl(KeyCode::Char('c')).into()),
        }
    }
}

/// Starts a game with a given [`Controller`], which refreshes at the given tick_duration (a [`Duration`]).
pub fn run_game(controller: &mut dyn Controller, settings: GameSettings) -> Result<(), GameError> {
    // setup terminal
    enable_raw_mode().map_err(|e| GameError::RawMode(Box::new(e)))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(GameError::TerminalExecute)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(GameError::TerminalMode)?;

    // create app and run it
    let res = run_gameloop(&mut terminal, controller, settings);

    // restore terminal
    disable_raw_mode().map_err(|e| GameError::RawMode(Box::new(e)))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(GameError::TerminalExecute)?;
    terminal.show_cursor().map_err(GameError::TerminalMode)?;

    res.map_err(GameError::Running)
}

/// Function is called internally once the terminal is configured,
/// and contains the event-loop.
///
/// This function does not clean up the terminal after itself,
/// it assumes that another function ([`run_game`]) will do that.
fn run_gameloop<B: Backend>(
    terminal: &mut Terminal<B>,
    controller: &mut dyn Controller,
    settings: GameSettings,
) -> io::Result<()> {
    let mut chunks: CharChunkMap = ChunkMap::new();
    let mut last_tick = Instant::now();
    let mut game = Game::new(&mut chunks);
    controller.on_start(&mut game);
    loop {
        {
            terminal.draw(|f| ui(f, &game))?;
        }
        let timeout = settings
            .tick_duration
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if poll(timeout)? {
            let event = event::read()?;
            if let Some(quit_event) = settings.quit_event.as_ref() {
                if &event == quit_event {
                    return Ok(());
                }
            }
            controller.on_event(&mut game, event);
        }
        if game.game_will_end() {
            return Ok(());
        }

        if last_tick.elapsed() >= settings.tick_duration {
            controller.on_tick(&mut game);
            last_tick = Instant::now();

            if game.game_will_end() {
                return Ok(());
            }
        }
    }
}

/// Creates a block for the [`ui`] function, with the given title.
fn create_block(title: Option<String>) -> tui::widgets::Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(GameColor::White).fg(GameColor::Black))
        .title(Span::styled(
            title.unwrap_or_else(|| "Message".to_string()),
            Style::default().add_modifier(Modifier::BOLD),
        ))
}

/// Creates the UI for a particular level.
fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
    if f.size().height < SCREEN_HEIGHT || f.size().width < SCREEN_WIDTH {
        let text = vec![Spans::from(Span::styled(
            format!("cs6991's Explorer requires a {SCREEN_HEIGHT}x{SCREEN_WIDTH} terminal!"),
            Style::default().fg(GameColor::Red),
        ))];
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Error").borders(Borders::ALL))
            .style(Style::default().bg(GameColor::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, f.size());
    } else {
        let size = f.size();

        let (width, (main_height, msg_height)) = game.screen_size();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(size.width.saturating_sub(SCREEN_WIDTH) / 2),
                    Constraint::Length(width),
                    Constraint::Length(size.width.saturating_sub(SCREEN_WIDTH) / 2),
                ]
                .as_ref(),
            )
            .split(size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(size.height.saturating_sub(SCREEN_HEIGHT) / 2),
                    Constraint::Length(main_height),
                    Constraint::Length(msg_height),
                    Constraint::Length(size.height.saturating_sub(SCREEN_HEIGHT) / 2),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let charview = CharView::new(game.chunks)
            .viewport(game.get_viewport())
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(charview, chunks[1]);

        if let Some(msg) = game.get_message() {
            let paragraph = Paragraph::new(msg.text.clone().replace('\t', "  "))
                .style(Style::default().bg(GameColor::White).fg(GameColor::Black))
                .block(create_block(msg.title.clone()))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[2]);
        }
    }
}
