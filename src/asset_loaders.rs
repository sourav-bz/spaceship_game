use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets{
   pub asteroid: Handle<Scene>,
   pub spaceship: Handle<Scene>,
   pub rock: Handle<Scene>,
}

pub struct AssetLoadersPlugin;

impl Plugin for AssetLoadersPlugin{
    fn build(&self, app: &mut App){
        app.init_resource::<SceneAssets>();
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(mut asset_server: ResMut<AssetServer>, mut scene_assets: ResMut<SceneAssets>){
    scene_assets.asteroid = asset_server.load("Asteroid.glb#Scene0");
    scene_assets.spaceship = asset_server.load("Spaceship.glb#Scene0");
    scene_assets.rock = asset_server.load("Bullets.glb#Scene0");
}