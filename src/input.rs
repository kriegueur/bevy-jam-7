use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(InputVector(Vec2::new(0., 0.)))
            .insert_resource(ShootingInput(false))
            .add_systems(Update, read_input);
    }
}

#[derive(Resource, derive_more::Deref, derive_more::DerefMut)]
pub struct InputVector(pub Vec2);

#[derive(Resource, derive_more::Deref, derive_more::DerefMut)]
pub struct ShootingInput(pub bool);

const SHOOTING_KEY: KeyCode = KeyCode::Space;
fn read_input(
    mut input_resource: ResMut<InputVector>,
    mut shooting_resource: ResMut<ShootingInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        input_resource.y = 1.;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        input_resource.y = -1.;
    } else {
        input_resource.y = 0.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        input_resource.x = -1.;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        input_resource.x = 1.;
    } else {
        input_resource.x = 0.;
    }

    input_resource.0 = input_resource.normalize_or_zero();

    if keyboard_input.pressed(SHOOTING_KEY) {
        shooting_resource.0 = true;
    } else {
        shooting_resource.0 = false;
    }
}
