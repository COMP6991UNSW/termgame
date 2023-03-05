#![warn(missing_docs)]

use divrem::DivFloor;
use std::collections::HashMap;

/// Chunks in the ChunkMap are always `CHUNK_SIZE x CHUNK_SIZE`.
///
/// Larger chunk sizes mean more memory is used; but also mean
/// less lookups of the HashMap.
const CHUNK_SIZE: usize = 64;
const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

type Chunk<T> = [[T; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkCoordinate {
    x: i32,
    y: i32,
}

impl ChunkCoordinate {
    fn get_from_coordinates(x: i32, y: i32) -> ChunkCoordinate {
        ChunkCoordinate {
            x: DivFloor::div_floor(x, CHUNK_SIZE_I32) * CHUNK_SIZE_I32,
            y: DivFloor::div_floor(y, CHUNK_SIZE_I32) * CHUNK_SIZE_I32,
        }
    }

    fn x_offset(&self, x: i32) -> usize {
        let offset = x - self.x;
        if (0..CHUNK_SIZE_I32).contains(&offset) {
            return offset as usize;
        }
        panic!("Cannot find x_offset within this chunk!")
    }

    fn y_offset(&self, y: i32) -> usize {
        let offset = y - self.y;
        if (0..CHUNK_SIZE_I32).contains(&offset) {
            return offset as usize;
        }
        panic!("Cannot find y_offset within this chunk!")
    }
}

/// A ChunkMap is an infinite 2D plane consisting of elements of
/// type `T`. It's basically a [`HashMap`], but using a `HashMap`
/// would have terrible iteration performance; so it stores
/// values in "chunks". Each Chunk is stored in a HashMap,
/// and individual elements are accessed by finding the address
/// of their chunk, then getting them by offset.
#[derive(Debug, Clone)]
pub struct ChunkMap<T> {
    map: HashMap<ChunkCoordinate, Chunk<Option<T>>>,
}

impl<T: Copy> ChunkMap<T> {
    /// Creates a new [`ChunkMap`]
    pub fn new() -> ChunkMap<T> {
        ChunkMap {
            map: HashMap::new(),
        }
    }

    /// Returns a mutable reference to an [`Option<T>`], which
    /// is the slot for `(x, y)`. This is always a mutating operation,
    /// as even if the chunk for `(x, y)` has not been created yet; this
    /// must create it.
    fn get_slot(&mut self, x: i32, y: i32) -> &mut Option<T> {
        let coord = ChunkCoordinate::get_from_coordinates(x, y);
        &mut self
            .map
            .entry(coord)
            .or_insert_with(|| [[None; CHUNK_SIZE]; CHUNK_SIZE])[coord.x_offset(x)]
            [coord.y_offset(y)]
    }

    /// Returns an Optional reference to the `T` at `(x, y)` if there
    /// is one.
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        let coord = ChunkCoordinate::get_from_coordinates(x, y);
        self.map.get(&coord)?[coord.x_offset(x)][coord.y_offset(y)].as_ref()
    }

    /// Removes the `T` at `(x, y)` if there was one, and returns
    /// it as an `Option<T>`. If the option is `None`, it indicates
    /// nothing was there. This only mutates if `(x, y)` has something
    /// present.
    pub fn remove(&mut self, x: i32, y: i32) -> Option<T> {
        let coord = ChunkCoordinate::get_from_coordinates(x, y);
        // If the chunk doesn't exist, there can't be anything to remove.
        let chunk = self.map.get_mut(&coord)?;
        let value = &mut chunk[coord.x_offset(x)][coord.y_offset(y)];
        value.take()
    }

    /// Inserts `val` at `(x, y)`.
    pub fn insert(&mut self, x: i32, y: i32, val: T) {
        *self.get_slot(x, y) = Some(val);
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkCoordinate;
    use super::ChunkMap;

    #[test]
    fn check_chunk_coordinate() {
        let c = ChunkCoordinate::get_from_coordinates(3, 3);
        assert!(c.x == 0);
        assert!(c.y == 0);
        let c = ChunkCoordinate::get_from_coordinates(72, 3);
        assert!(c.x == 64);
        assert!(c.y == 0);
    }

    #[test]
    fn get_none_chunkmap() {
        let c = ChunkMap::<i32>::new();
        assert_eq!(c.get(0, 0), None);
    }

    #[test]
    fn get_some_chunkmap() {
        let mut c = ChunkMap::<i32>::new();
        c.insert(3, 3, 7);
        assert_eq!(c.get(3, 3), Some(&7));
        c.insert(103, 103, 7);
        assert_eq!(c.get(103, 103), Some(&7));
    }
    #[test]
    fn check_chunkmap_default() {
        let mut c = ChunkMap::<i32>::new();
        c.insert(4, 3, 0);
        assert_eq!(c.get(4, 3), Some(&0));
        assert_eq!(c.get(5, 3), None);
        assert_eq!(c.get(65, 3), None);
    }
}
