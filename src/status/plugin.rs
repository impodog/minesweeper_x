use crate::prelude::*;

use super::event::FlipEvent;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameStartEvent>()
            .add_event::<GameEndEvent>()
            .add_event::<SpawnMenuEvent>()
            .add_event::<KillMenuEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<FlipEvent>();
    }
}
