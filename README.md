# termgame

TermGame is a small crate that extends tui-rs to make it easy
to write TUI games.

Mainly used in COMP6991 at the University of New South Wales,
the crate provides the [`Controller`] trait, which is accepted
by the [`run_game`] function to start a game using a Crossterm
TUI (provided by ratatui).

It also wraps many tui features, like [`StyledCharacter`],
[`GameEvent`], and [`Style`].

```rust
use termgame::{SimpleEvent, Controller, Game, GameEvent, StyledCharacter, run_game, KeyCode};
use std::error::Error;
use std::time::Duration;

struct MyGame {}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                game.set_screen_char(1, 1, Some(StyledCharacter::new(ch)))
            },
            _ => {}
        }

    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame {};

    run_game(
        &mut controller,
        GameSettings::new().tick_duration(Duration::from_millis(500))
    )?;

    println!("Game Ended!");

    Ok(())
}
```

License: MIT OR Apache-2.0
