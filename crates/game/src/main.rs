use bevy::prelude::*;
use neoforge_core::hello_core;

fn main() {
    println!("{}", hello_core());
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, hello)
        .run();
}

fn hello() {
    info!("NeoForge Legends Game Booting");
}
