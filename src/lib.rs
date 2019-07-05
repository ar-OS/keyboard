#![no_std]

// Choose which scancode set to use
pub mod set;

use set::KeyType;
use set::Set;

/// Keyboard stores events & choosed set
pub struct Keyboard<S>
where
    S: Set,
{
    set: S,
}

impl<S> Keyboard<S>
where
    S: Set,
{
    /// Allows to construct a new Keyboard object
    /// based on a given scancode set
    pub fn new(set: S) -> Keyboard<S> {
        Keyboard { set: set }
    }
    /// Allows to append a new keycode
    /// and to retrieve the good scancode
    /// based on the context
    pub fn push(&mut self, scancode: u8) -> Option<KeyType> {
        self.set.match_scancode(scancode)
    }
}
