use ::minesweeper_x::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ResourcePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(StatusPlugin)
        .add_plugins(GamePlugin)
        .run();
}
