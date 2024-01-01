use crate::prelude::*;

fn system_jump_start_game(
    mut state: ResMut<NextState<GameState>>,
    mut event_game: EventWriter<GameEndEvent>,
) {
    state.set(GameState::Menu);
    event_game.send(GameEndEvent {});
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (system_jump_start_game, system_camera))
            .add_systems(Update, (system_spawn_map, system_despawn_map));
    }
}
