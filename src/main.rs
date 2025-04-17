mod asset_loaders;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loaders::AssetLoadersPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoadersPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
