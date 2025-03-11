

use bevy::prelude::*;

use crate::asset_loaders::SceneAssets;
use crate::movement::{Velocity, Acceleration, MovingObjectBundle};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
        app.add_systems(Update, spaceship_movement);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>){
    let scene = scene_assets.spaceship.clone();
    commands.spawn((MovingObjectBundle{
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration{value: Vec3::ZERO},
        scene: SceneRoot(scene),
        transform: Transform::from_translation(STARTING_TRANSLATION),
    }, Spaceship));
}

fn spaceship_movement(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>){
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;
    
    if keyboard_input.pressed(KeyCode::KeyA){
        rotation += SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }else if keyboard_input.pressed(KeyCode::KeyD){
        rotation -= SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyW){
        movement += SPACESHIP_SPEED;
    }else if keyboard_input.pressed(KeyCode::KeyS){
        movement -= SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyQ){
        roll += SPACESHIP_ROLL_SPEED * time.delta_secs();
    }else if keyboard_input.pressed(KeyCode::KeyE){
        roll -= SPACESHIP_ROLL_SPEED * time.delta_secs();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;

}