/// These are different reasons a game could end.
#[derive(Debug)]
pub enum GameError {
    /// An error occured while running the application.
    Running(std::io::Error),
    /// An error occured trying to enable RawMode on
    /// this terminal.
    RawMode(Box<dyn std::error::Error>),
    /// An error occured while creating a new terminal or showing the cursor.
    TerminalMode(std::io::Error),
    /// An error occured trying to execute commands on the terminal.
    TerminalExecute(std::io::Error),
}

impl std::error::Error for GameError {}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game Encountered an Error: {:?}", self) // user-facing output
    }
}
