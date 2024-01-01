use crate::prelude::*;

#[derive(Event)]
pub struct GameStartEvent {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

#[derive(Event)]
pub struct GameEndEvent {}

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
