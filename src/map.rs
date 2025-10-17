use bevy::prelude::*;

pub fn render_map(mut commands: Commands) {
    let _map = vec![vec![0; 32]; 32]; // 0=terrain, 1=ruin
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        ..default()
    });
}
