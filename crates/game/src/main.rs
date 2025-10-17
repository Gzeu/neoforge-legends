use bevy::prelude::*;
use neoforge_core::{hello_core, Vec2 as GameVec2};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Game state and components
#[derive(Resource, Default)]
struct GameState {
    score: u32,
    player_pos: GameVec2,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(GameVec2);

// Systems
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    
    // UI - Score display
    commands.spawn((
        TextBundle::from_section(
            "NeoForge Legends - Score: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));
    
    // Player sprite (simple colored square for MVP)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.8, 1.0), // Cyan player
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
        Velocity(GameVec2 { x: 0.0, y: 0.0 }),
    ));
    
    info!("NeoForge Legends Game Setup Complete");
}

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = query.single_mut();
    let speed = 200.0;
    
    velocity.0.x = 0.0;
    velocity.0.y = 0.0;
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        velocity.0.x = -speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        velocity.0.x = speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        velocity.0.y = speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        velocity.0.y = -speed;
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    mut game_state: ResMut<GameState>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
        
        // Update game state position
        game_state.player_pos.x = transform.translation.x;
        game_state.player_pos.y = transform.translation.y;
        
        // Simple score increment based on movement
        if velocity.0.x.abs() > 0.0 || velocity.0.y.abs() > 0.0 {
            game_state.score += (time.delta_seconds() * 10.0) as u32;
        }
    }
}

fn update_ui(
    game_state: Res<GameState>,
    mut query: Query<&mut Text>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "NeoForge Legends - Score: {} | Pos: ({:.1}, {:.1})",
            game_state.score,
            game_state.player_pos.x,
            game_state.player_pos.y
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    println!("{}", hello_core());
    
    let mut app = App::new();
    
    // Configure plugins based on target
    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "NeoForge Legends".into(),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "NeoForge Legends".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }));
    }
    
    app
        .init_resource::<GameState>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, (player_input, move_player, update_ui))
        .run();
}
