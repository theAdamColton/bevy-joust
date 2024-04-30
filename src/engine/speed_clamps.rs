/*
Allows inserting components that clamp velocity components
 */

use bevy::prelude::*;
use crate::engine::physics::{Velocity, PhysicsStages};

pub struct SpeedClampPlugin;
impl Plugin for SpeedClampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,clamp_velocities.after(PhysicsStages::CalculateNextVelocities));
    }
}

#[derive(Component)]
pub struct SpeedClamps {
    pub pos_x: f32,
    pub neg_x: f32,
    pub pos_y: f32,
    pub neg_y: f32,
}
impl SpeedClamps {
    pub fn new_from_x_y(x: f32, y: f32) -> Self {
        Self {
            pos_x: x,
            neg_x: -x, 
            pos_y: y,
            neg_y: -y,
        }
    }

    pub fn new_from_single(a: f32) -> Self {
        Self::new_from_x_y(a, a)
    }
}

fn clamp_velocities(mut q: Query<(&mut Velocity, &SpeedClamps)>) {
    for (mut vel, sc) in q.iter_mut() {
        vel.y = vel.y.clamp(sc.neg_y, sc.pos_y);
        vel.x = vel.x.clamp(sc.neg_x, sc.pos_x);
    }
}
