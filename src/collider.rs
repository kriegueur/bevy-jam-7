use bevy::math::bounding::{BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component)]
pub struct SphereCollider {
    radius: f32,
}

impl SphereCollider {
    pub fn new(radius : f32) -> Self {
        Self {radius : radius}
    }
}

fn sphere_collider_collides(
    collider1: &SphereCollider,
    transform1: Transform,
    collider2: &SphereCollider,
    transform2: Transform,
) -> bool {
    let circle_bound1: BoundingCircle = BoundingCircle::new(
        Vec2::new(transform1.translation.x, transform1.translation.y),
        collider1.radius,
    );
    let circle_bound2: BoundingCircle = BoundingCircle::new(
        Vec2::new(transform2.translation.x, transform2.translation.y),
        collider2.radius,
    );
    circle_bound1.intersects(&circle_bound2)
}
