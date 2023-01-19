use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};
use bevy::prelude::*;

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
    });
}
