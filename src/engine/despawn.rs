/*
 * Insert a despawn timer to get an entity to despawn after the timer runs up.
 */

use bevy::prelude::*;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,despawn);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(pub Timer);

fn despawn(mut commands: Commands, time: Res<Time>, mut q: Query<(&mut DespawnTimer, Entity)>) {
    for (mut timer, ent) in q.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(ent).despawn_recursive();
        }
    }
}

