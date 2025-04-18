use bevy::prelude::*;
use rand::random_range;
use std::ops::RangeInclusive;

use crate::asset_loaders::SceneAssets;
use crate::collision_detection::{Collider, CollisionDamage};
use crate::health::Health;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: RangeInclusive<f32> = -10.0..=10.0;
const SPAWN_RANGE_Z: RangeInclusive<f32> = 0.0..=25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;
const ASTEROID_HEALTH: f32 = 80.0;
const ASTEROID_COLLISION_DAMAGE: f32 = 35.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        });
        // app.add_systems(Update, spawn_asteroid);
        // app.add_systems(Update, handle_asteroid_collisions);
        // app.add_systems(Update, rotate_asteroids);

        app.add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let translation = Vec3::new(
        random_range(SPAWN_RANGE_X),
        0.0,
        random_range(SPAWN_RANGE_Z),
    );

    let random_unit_vector =
        Vec3::new(random_range(-1.0..=1.0), 0.0, random_range(-1.0..=1.0)).normalize_or_zero();

    let velocity = random_unit_vector * VELOCITY_SCALAR;
    let acceleration = random_unit_vector * ACCELERATION_SCALAR;

    let asteroid = scene_assets.asteroid.clone();
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration {
                value: acceleration,
            },
            scene: SceneRoot(asteroid),
            transform: Transform::from_translation(translation),
            collider: Collider::new(RADIUS),
        },
        Asteroid,
        Health::new(ASTEROID_HEALTH),
        CollisionDamage::new(ASTEROID_COLLISION_DAMAGE),
    ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}

// fn handle_asteroid_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Asteroid>>){
//     for (entity, collider) in query.iter(){
//         for &collided_entity in collider.colliding_entities.iter(){
//             if query.get(collided_entity).is_ok(){
//                 continue;
//             }

//             commands.entity(entity).despawn();
//         }
//     }
// }
