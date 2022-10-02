#![warn(missing_docs)]

use super::game::{Game, GameEvent};

/// The [`Controller`] trait must be implemented on a struct
/// in order to control a Termgame Game.
///
/// Each of the event-handlers (i.e. functions) will be called as they
/// are descrbed, and passed a [`Game`] trait object; which gives you
/// functions to control the game.
pub trait Controller {
    /// This event-handler is called just before any other event-handlers.
    /// You should use it for any initialisation that needs to happen once
    /// per controller (though you could also initialise things before
    /// this is called if you prefer).
    fn on_start(&mut self, game: &mut Game);

    /// Whenever the user interacts with the Game, this event-handler will
    /// be called and the relevant [`GameEvent`] will be provided. Use this
    /// function to handle key-presses from the user.
    fn on_event(&mut self, game: &mut Game, event: GameEvent);

    /// This function is called between every time the Termgame is drawn.
    /// It allows you to make actions happen independently of user-input.
    fn on_tick(&mut self, game: &mut Game);
}
