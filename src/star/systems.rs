use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::Data;

use super::components::Star;
use super::resources::*;
use super::NUMBER_OF_STARS;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    data: ResMut<Data>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: {
                    let mut transf = Transform::from_xyz(random_x, random_y, 0.0);
                    transf.scale = Vec3::new(2.0, 2.0, 2.0);
                    transf
                },
                texture: asset_server.load({
                    if data.is_night {
                        "sprites/coin_night.png"
                    } else {
                        "sprites/coin_day.png"
                    }
                }),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    data: ResMut<Data>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: {
                    let mut transf = Transform::from_xyz(random_x, random_y, 0.0);
                    transf.scale = Vec3::new(2.0, 2.0, 2.0);
                    transf
                },
                texture: asset_server.load({
                    if data.is_night {
                        "sprites/coin_night.png"
                    } else {
                        "sprites/coin_day.png"
                    }
                }),
                ..default()
            },
            Star {},
        ));
    }
}
