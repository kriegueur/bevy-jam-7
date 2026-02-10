use crate::{collider::SphereCollider, input::ShootingInput, projectile::spawn_projectile};

use super::input::InputVector;

use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShootingTimer(Timer::from_seconds(SHOOTING_COOLDOWN, TimerMode::Once)))
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, (move_player, shoot));
        // add things to your app here
    }
}

const PLAYER_SPEED: f32 = 100.;
const SHOOTING_COOLDOWN : f32 = 0.25;
const SHOOTING_OFFSET: Vec2 = Vec2::new(0., 20.);
const SHOOTING_DIRECTION: Vec2 = Vec2::new(0., 1.);
const SHOOTING_SPEED: f32 = 500.;
const SHOOTING_RADIUS: f32 = 2.;


#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 0.),
        Sprite::from_color(WHITE, Vec2::new(10., 10.)),
    ));
}

fn move_player(
    input: Res<InputVector>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    player_transform.translation.x += input.x * PLAYER_SPEED * time.delta().as_secs_f32();
    player_transform.translation.y += input.y * PLAYER_SPEED * time.delta().as_secs_f32();
}

#[derive(Resource)]
struct ShootingTimer(Timer);

fn shoot(
    mut commands: Commands,
    input : Res<ShootingInput>,
    mut timer: ResMut<ShootingTimer>,
    time : Res<Time>,
    player_transform : Single<&Transform, With<Player>>
) {
    if !timer.0.is_finished() {
        timer.0.tick(time.delta());
    }
    else if input.0 {
        // Generate projectile
        let shot_origin = Vec2::new(player_transform.translation.x + SHOOTING_OFFSET.x,
            player_transform.translation.y + SHOOTING_OFFSET.y);
        let shot_velocity = (SHOOTING_DIRECTION * SHOOTING_SPEED).into();
        let shot_collider = SphereCollider::new(SHOOTING_RADIUS);
        spawn_projectile(&mut commands, shot_origin, shot_velocity, shot_collider);

        // Start timer
        timer.0.reset();
    }
}
