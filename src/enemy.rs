use crate::components::{Enemy, SpriteSize};
use crate::{GameTextures, WinSize, ENEMY_SIZE, SPRITE_SCALE};
use bevy::prelude::*;
use rand::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut cmds: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);
    cmds.spawn(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..Default::default()
    })
    .insert(Enemy)
    .insert(SpriteSize::from(ENEMY_SIZE));
}
