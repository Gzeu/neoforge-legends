use bevy::prelude::*;
mod player;
mod crafting;
mod combat;
mod map;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(player::setup_player)
        .add_system(crafting::craft_tool)
        .add_system(combat::attack_enemy)
        .add_system(map::render_map)
        .add_system(ui::ui_system)
        .run();
}
