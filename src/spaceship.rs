use bevy::prelude::*;

use crate::asset_loaders::SceneAssets;
use crate::movement::{Velocity, Acceleration, MovingObjectBundle};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>){
    let scene = scene_assets.spaceship.clone();
    commands.spawn(MovingObjectBundle{
        velocity: Velocity{value: STARTING_VELOCITY},
        acceleration: Acceleration{value: Vec3::ZERO},
        scene: SceneRoot(scene),
        transform: Transform::from_translation(STARTING_TRANSLATION),
    });
}