use crate::prelude::*;

pub struct GamePlugin;

fn when_play(state: Res<State<GameState>>) -> bool {
    *state == GameState::Game
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (system_mouse_listener, system_flip, system_redraw_dirty).run_if(when_play),
        );
    }
}
