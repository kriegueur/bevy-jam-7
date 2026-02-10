use crate::{collider::SphereCollider, velocity::*};
use bevy::prelude::*;

pub struct ProjectilePlugin;


#[derive(Component)]
pub struct Projectile;

pub fn spawn_projectile(
    commands: &mut Commands,
    origin: Vec2,
    velocity: Velocity,
    collider: SphereCollider,
) {
    commands.spawn((
        Projectile,
        Transform::from_xyz(origin.x, origin.y, 0.),
        Sprite::from_color(Color::srgb(0.5, 0.5, 0.5), Vec2::new(5., 5.)),
        velocity,
        collider,
    ));
}
