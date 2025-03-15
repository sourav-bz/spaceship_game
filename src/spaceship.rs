use bevy::{prelude::*, transform};

use crate::asset_loaders::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Velocity, Acceleration, MovingObjectBundle};
use crate::schedule::InGameSet;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);

        app.add_systems(
            Update, 
            (
                spaceship_movement, 
                spaceship_weapon_controls, 
                spaceship_shield_controls
            )
            .chain()
            .in_set(InGameSet::UserInput)
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>){
    let scene = scene_assets.spaceship.clone();
    commands.spawn((MovingObjectBundle{
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration{value: Vec3::ZERO},
        scene: SceneRoot(scene),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        collider: Collider::new(SPACESHIP_RADIUS)
    }, Spaceship));
}

fn spaceship_movement(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>){

    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

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

fn spaceship_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>, scene_assets: Res<SceneAssets>){
    let Ok(transform) = query.get_single() else {
        return;
    };
    
    let scene = scene_assets.rock.clone();
    if keyboard_input.pressed(KeyCode::Space){
        commands.spawn((MovingObjectBundle{
            velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            scene: SceneRoot(scene),
            transform: Transform::from_translation(transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR),
            collider: Collider::new(MISSILE_RADIUS)
        }, SpaceshipMissile));
    }
}

fn spaceship_shield_controls(mut commands: Commands, query: Query<Entity, With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>){
    let Ok(spaceship) = query.get_single() else{
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab){
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}