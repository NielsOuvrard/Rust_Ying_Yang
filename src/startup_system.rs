use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::Enemy;
use crate::Player;
use crate::NUMBER_OF_ENEMIES;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_ennemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// use Enemy;
// use Player;

// pub const PLAYER_SPEED: f32 = 500.0;
// pub const PLAYER_SIZE: f32 = 64.0;
// pub const NUMBER_OF_ENEMIES: usize = 10;
// pub const ENEMY_SPEED: f32 = 200.0;
// pub const ENEMY_SIZE: f32 = 64.0;

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture_handle = asset_server.load("sprites/tilemap.png");
//     let texture_atlas =
//         TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 1, None, None);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 1, last: 6 };
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn((
//         SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle,
//             sprite: TextureAtlasSprite::new(animation_indices.first),
//             transform: Transform::from_scale(Vec3::splat(6.0)),
//             ..default()
//         },
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//     ));
// }
