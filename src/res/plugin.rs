use crate::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_init_assets)
            .add_systems(PostStartup, system_init_window);
    }
}
