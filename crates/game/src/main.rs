use bevy::prelude::*;
use neoforge_core::hello_core;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    println!("{}", hello_core());
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "NeoForge Legends".into(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_systems(Startup, hello).run();
}

fn hello() {
    info!("NeoForge Legends Game Booting");
}
