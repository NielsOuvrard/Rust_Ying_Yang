pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use std::env;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::{prelude::*, window::PresentMode};

fn main() {
    let color_app = {
        if env::args().len() > 1 {
            println!("nmb of args: {}", env::args().len());
            env::args().nth(1).unwrap().to_string()
        } else {
            "night".to_string()
        }
    };
    App::new()
        .insert_resource(ClearColor({
            if color_app == "white" || color_app == "day" {
                Color::WHITE
            } else {
                Color::BLACK
            }
        }))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Window!".into(),
                        resolution: (1260., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Data {
            is_night: color_app == "night",
        })
        // .add_startup_system(load_tilemap)
        .add_event::<GameOver>()
        .add_system(change_day_night)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(Resource)]
pub struct Data {
    // pub texture_atlas_handle: Handle<TextureAtlas>,
    pub is_night: bool,
}

// impl Default for Data {
//     fn default() -> Data {
//         Data { is_night: false }
//     }
// }

// pub fn load_tilemap(
// asset_server: Res<AssetServer>,
// mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     data: ResMut<Data>,
// ) {
// let texture_handle = asset_server.load("sprites/tilemap.png");

// let texture_atlas =
//     TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 20, 20, None, None);

// data.texture_atlas_handle = texture_atlases.add(texture_atlas);

// let window = window_query.get_single().unwrap();

// commands.spawn((
//     SpriteSheetBundle {
//         transform: {
//             let mut transform =
//                 Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
//             transform.scale = Vec3::new(3, 3, 3);
//             transform
//         },
//         texture_atlas: texture_atlas_handle.clone(),
//         sprite: TextureAtlasSprite::new(240),
//         ..default()
//     },
//     Player {
//         frame: 0,
//         action: ActionPlayer::Idle,
//     },
// ));
// }
