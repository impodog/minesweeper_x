pub use bevy::prelude::*;

pub use rand::Rng;

pub use crate::game::*;
pub use crate::map::*;
pub use crate::menu::*;
pub use crate::res::*;
pub use crate::spawn::*;
pub use crate::status::*;

pub static MENU_SELECTION: &'static [&'static str] = &["Classic", "Flagger"];
pub static MENU_SIZE: f32 = 100.0;
