use super::input::InputVector;

use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player);
        // add things to your app here
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands : Commands) {
    commands.spawn((Player, Transform::from_xyz(0., 0., 0.), Sprite::from_color(WHITE, Vec2::new(10., 10.))));
}

const PLAYER_SPEED : f32 = 100.;
fn move_player(input : Res<InputVector>,
    mut player_transform : Single<&mut Transform, With<Player>>,
    time : Res<Time>) {
    player_transform.translation.x += input.x * PLAYER_SPEED * time.delta().as_secs_f32();
    player_transform.translation.y += input.y * PLAYER_SPEED * time.delta().as_secs_f32();
}