mod input;
mod player;

use {
    input::InputPlugin,
    player::PlayerPlugin
};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (setup_window, setup_camera, (spawn_boss, set_boss_size).chain()))
        .run();
}

const WINDOW_WIDTH : f32 = 640.;
const WINDOW_HEIGHT : f32 = 480.;
fn setup_window(mut window : Single<&mut Window>) {
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);
}

fn setup_camera(mut commands : Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Boss;

fn spawn_boss(mut commands : Commands, asset_server : Res<AssetServer>) {
    commands.spawn((Boss,
        Sprite::from_image(asset_server.load("lambo.png")),
        Transform::from_xyz(0., 100., 0.)));
}

fn set_boss_size(mut boss_sprite : Single<&mut Sprite, With<Boss>>) {
    boss_sprite.custom_size = Some((100., 100.).into());
}