use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

fn apply_velocity(query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query {
        transform.translation.x += velocity.x * time.delta().as_secs_f32();
        transform.translation.y += velocity.y * time.delta().as_secs_f32();
    }
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}