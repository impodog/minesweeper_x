use crate::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    Game,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Menu
    }
}
