mod spaceship;
mod movement;
mod debug;
mod camera;
use bevy::prelude::*;
use spaceship::SpaceshipPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;
fn main(){
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight{
            color: Color::default(),
            brightness: 750.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();
}