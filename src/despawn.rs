use bevy::prelude::*;

const DESPWAN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>){
    for (entity, transform) in query.iter(){
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > DESPWAN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}