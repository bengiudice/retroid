use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

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
        .add_startup_system(setup_system)
        .run()
}

fn setup_system(mut cmds: Commands, mut windows: ResMut<Windows>, asset_server: Res<AssetServer>) {
    cmds.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    window.set_position(MonitorSelection::Current, IVec2::new(2780, 4900));
    let window_width = window.width();
    let window_height = window.height();
    let bottom = -window_height / 2.;

    cmds.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 5., 10.),
            ..default()
        },
        ..default()
    });
}
