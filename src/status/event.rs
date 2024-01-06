use crate::prelude::*;

#[derive(Debug, Event)]
pub struct GameStartEvent {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub mode: GameMode,
}

#[derive(Event)]
pub struct SpawnMenuEvent;

#[derive(Event)]
pub struct KillMenuEvent;

#[derive(Event)]
pub struct GameEndEvent {}

#[derive(Event)]
pub struct GameOverEvent {
    pub win: bool,
}

pub enum FlipType {
    Open,
    Mark,
    OpenAll,
}

#[derive(Event)]
pub struct FlipEvent {
    pub button: FlipType,
    pub x: usize,
    pub y: usize,
}
