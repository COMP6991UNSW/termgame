#![warn(missing_docs)]

use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

pub mod chunkmap;
pub mod screen_character;

use super::charview::{chunkmap::ChunkMap, screen_character::ScreenCharacter};

/// This is a [`ChunkMap`] (an infinite 2D map) of [`ScreenCharacter`]s
pub type CharChunkMap = ChunkMap<ScreenCharacter>;

/// The [`ViewportLocation`] describes the top-left
#[derive(Debug, Clone, Copy)]
pub struct ViewportLocation {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

/// A widget that shows a small view into an infinitely sized map.
#[derive(Debug, Clone)]
pub struct CharView<'a> {
    /// The actual data inside the CharView
    data: &'a CharChunkMap,
    /// The tui-rs [`Block`].
    block: Option<Block<'a>>,
    /// The leftmost x value shown in the viewport.
    viewport: ViewportLocation,
}

impl<'a> CharView<'a> {
    /// Creates a basic CharView
    pub fn new(data: &'a CharChunkMap) -> CharView<'a> {
        CharView {
            block: None,
            viewport: ViewportLocation { x: 0, y: 0 },
            data,
        }
    }

    /// Saves the tui-rs [`Block`] in this struct.
    pub fn block(mut self, block: Block<'a>) -> CharView<'a> {
        self.block = Some(block);
        self
    }

    /// Set the top-left corner of the widget to the given [`ViewportLocation`]'s
    /// `x` and `y` coordinates. Coordnates run down and to the right, as is
    /// normal on computer monitors.
    ///
    ///
    /// ```ignore
    ///
    /// let mut map = CharChunkMap::new();
    /// for i in 0..5 {
    ///     map.insert(i, i, 'a');
    /// }
    ///
    /// let at = ViewportLocation { x: 0, y: 0};
    /// ```
    ///
    /// ```text
    /// at=(0,0)| at=(0,1)| at=(1,0)| at=(2,0)
    ///  +---+  |  +---+  |  +---+  |  +---+
    ///  |a  |  |  | a |  |  |   |  |  |   |
    ///  | a |  |  |  a|  |  |a  |  |  |   |
    ///  |  a|  |  |   |  |  | a |  |  |a  |
    ///  +---+  |  +---+  |  +---+  |  +---+
    /// ```
    pub fn viewport(mut self, viewport: ViewportLocation) -> CharView<'a> {
        self.viewport = viewport;
        self
    }
}

impl<'a> Widget for CharView<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let charview_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if charview_area.height < 1 {
            return;
        }
        for y in charview_area.top()..charview_area.bottom() {
            for x in charview_area.left()..charview_area.right() {
                let shifted_x: i32 = (x - charview_area.left()) as i32 + self.viewport.x;
                let shifted_y: i32 = (y - charview_area.top()) as i32 + self.viewport.y;

                if let Some(screen_character) = self.data.get(shifted_x, shifted_y) {
                    buf.get_mut(x, y)
                        .set_char(screen_character.c)
                        .set_style((*screen_character).style.unwrap_or_default());
                }
            }
        }
    }
}
