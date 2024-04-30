use bevy::prelude::*;
use crate::engine::physics::{V2, PhysicsStages, Force, Mass, Acceleration};

/// Add this plugin to let ai and players control the movement of entities
pub struct MovementControlPlugin;
impl Plugin for MovementControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            apply_movement.in_set(PhysicsStages::CalculateNextForces).after(PhysicsStages::ClearNextForces));
    }
}

#[derive(Component, Copy, Clone)]
pub struct MovementControl(pub Acceleration);
impl Default for MovementControl {
    fn default() -> Self {
        Self(Acceleration(V2::new(0.0,0.0)))
    }
}

fn apply_movement(mut q: Query<(&mut Force, &MovementControl, &Mass)>){
    for (mut force, mc, mass) in q.iter_mut() {
        force.0 += mc.0.0 * mass.0;
    }
}
