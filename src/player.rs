use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
    ));
}
