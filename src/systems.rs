use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{
    enemy::components::Enemy, events::*, player::components::Player, star::components::Star, Data,
};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn change_day_night(
    mut data: ResMut<Data>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clear_color: ResMut<ClearColor>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        data.is_night = !data.is_night;
        clear_color.0 = if data.is_night {
            Color::BLACK
        } else {
            Color::WHITE
        };
        for (entity, transform) in star_query.iter_mut() {
            // Store the transform data
            let translation = transform.translation;
            let scale = transform.scale;

            // Despawn the current entity
            commands.entity(entity).despawn();

            // Spawn a new entity with the same transform
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load({
                        if data.is_night {
                            "sprites/coin_night.png"
                        } else {
                            "sprites/coin_day.png"
                        }
                    }),
                    transform: {
                        let mut transf = Transform::from_translation(translation);
                        transf.scale = scale;
                        transf
                    },
                    ..Default::default()
                },
                Star {},
            ));
        }
        for (entity, transform) in player_query.iter_mut() {
            // Store the transform data
            let translation = transform.translation;
            let scale = transform.scale;

            // Despawn the current entity
            commands.entity(entity).despawn();

            // Spawn a new entity with the same transform
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load({
                        if data.is_night {
                            "sprites/player_night.png"
                        } else {
                            "sprites/player_day.png"
                        }
                    }),
                    transform: {
                        let mut transf = Transform::from_translation(translation);
                        transf.scale = scale;
                        transf
                    },
                    ..Default::default()
                },
                Player {},
            ));
        }
        for (entity, transform, enemy) in enemy_query.iter_mut() {
            // Store the transform data
            let translation = transform.translation;
            let scale = transform.scale;
            // let enemy_data = entity

            // Despawn the current entity
            commands.entity(entity).despawn();

            // Spawn a new entity with the same transform
            commands.spawn((
                SpriteBundle {
                    texture: {
                        if data.is_night {
                            asset_server.load("sprites/enemy_night.png")
                        } else {
                            asset_server.load("sprites/enemy_day.png")
                        }
                    },
                    transform: {
                        let mut transf = Transform::from_translation(translation);
                        transf.scale = scale;
                        transf
                    },
                    ..Default::default()
                },
                Enemy { ..*enemy },
            ));
        }
    }
}
