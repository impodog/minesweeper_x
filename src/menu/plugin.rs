use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                system_input_box,
                system_start_game,
                system_end_game,
                system_spawn_input_box,
                system_despawn_input_box,
                system_end_game_event_listener,
                system_start_game_event_listener,
                system_selector,
                system_spawn_selector,
                system_despawn_selector,
            ),
        );
    }
}
