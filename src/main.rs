use crate::components::{Movable, Velocity};
use crate::player::PlayerPlugin;
use bevy::prelude::*;

mod components;
mod player;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Invaders!".to_string(),
                width: 598.0,
                height: 676.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(moveable_system)
        .run()
}

fn setup_system(mut cmds: Commands, mut windows: ResMut<Windows>, asset_server: Res<AssetServer>) {
    cmds.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    window.set_position(MonitorSelection::Current, IVec2::new(2780, 4900));
    let window_width = window.width();
    let window_height = window.height();

    let win_size = WinSize {
        w: window_width,
        h: window_height,
    };
    cmds.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    cmds.insert_resource(game_textures);
}

fn moveable_system(
    mut cmds: Commands,
    win_size: Res<WinSize>,
    mut q: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in q.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
