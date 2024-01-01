use crate::prelude::*;

#[derive(Event)]
pub struct GameStartEvent {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

#[derive(Event)]
pub struct SpawnInputBoxEvent;

#[derive(Event)]
pub struct DespawnInputBoxEvent;

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
