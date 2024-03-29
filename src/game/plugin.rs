use crate::prelude::*;

pub struct GamePlugin;

fn when_play(state: Res<State<GameState>>) -> bool {
    *state == GameState::Game
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, system_game_over);
        app.add_systems(
            PostUpdate,
            (
                system_mouse_listener,
                system_keyboard_listener,
                system_restart_game,
                system_flip,
                system_redraw_dirty,
            )
                .run_if(when_play),
        );
    }
}
