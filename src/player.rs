use crate::components::{Player, Velocity};
use crate::{GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut cmds: Commands,
    game_textures: Res<GameTextures>,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.;
    cmds.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Velocity { x: 0., y: 0. });
}

fn player_fire_system(
    mut cmds: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = q.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            cmds.spawn(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut q: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = q.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}

fn player_movement_system(mut q: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in q.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
