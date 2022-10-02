/// The [`Message`] struct is used when displaying
/// a message to a user.
pub struct Message {
    /// This will display a title in the top-left
    /// of the message. If not present, no title
    /// is shown.
    pub title: Option<String>,

    /// This is the main body of the message.
    pub text: String,
}

impl Message {
    /// Creates a message with the given text.
    pub fn new(text: String) -> Message {
        Message { title: None, text }
    }

    /// Builder method to add a title to an existing Message.
    ///
    /// ```rust
    /// use termgame::Message;
    /// Message::new(String::from("MyMessage"))
    ///          .title(String::from("Title"));
    /// ```
    pub fn title(mut self, title: String) -> Message {
        self.title = Some(title);
        self
    }
}
