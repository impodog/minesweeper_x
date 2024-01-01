use ::minesweeper_x::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(text_input::TextInputPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(ResourcePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(StatusPlugin)
        .add_plugins(GamePlugin)
        .run();
}
