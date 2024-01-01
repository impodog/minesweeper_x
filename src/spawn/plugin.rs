use crate::prelude::*;

fn debug_jump_start_game(
    mut state: ResMut<NextState<GameState>>,
    mut ev_game_start: EventWriter<GameStartEvent>,
) {
    state.set(GameState::Game);
    ev_game_start.send(GameStartEvent {
        width: 9,
        height: 9,
        mines: 10,
    });
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_camera)
            .add_systems(Update, (system_spawn_map, system_despawn_map));
        #[cfg(debug_assertions)]
        app.add_systems(PostStartup, debug_jump_start_game);
    }
}
