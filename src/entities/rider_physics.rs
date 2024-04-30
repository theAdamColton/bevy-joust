use crate::engine::collision::{Grounded, GroundedState};
use crate::behavior::movement_control::MovementControl;
use crate::engine::physics::{Acceleration, Force, PhysicsStages, Velocity};
use crate::animation::rider_animation::RiderAnimationState;
use bevy::prelude::*;

/// Determines aspects that effect the feel and speed of movement
#[derive(Component, Copy, Clone)]
pub struct RiderSpeedCharacteristics {
    /// Multiplied by the left right control axis (which is between -1 and 1)
    pub ground_acceleration: f32,
    pub ground_top_speed: f32,
    pub air_force: f32,
    pub air_top_speed_x: f32,
    pub air_top_speed_y: f32,
    pub jump_acceleration_from_air: f32,
    pub jump_acceleration_from_ground: f32,
}
impl RiderSpeedCharacteristics {
    pub fn default() -> Self {
        Self {
            ground_acceleration: 50.0,
            ground_top_speed: 30.0,
            air_force: 15.0,
            air_top_speed_x: 15.0,
            air_top_speed_y: 30.0,
            jump_acceleration_from_air: 300.0,
            jump_acceleration_from_ground: 700.0,
        }
    }
}

pub struct RiderPhysicsPlugin;
impl Plugin for RiderPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cap_velocities.after(PhysicsStages::CalculateNextVelocities));
        app.add_systems(
            Update,
            friction
                .in_set(PhysicsStages::CalculateNextForces)
                .after(PhysicsStages::ClearNextForces),
        );
    }
}

fn cap_velocities(mut q: Query<(&RiderSpeedCharacteristics, &Grounded, &mut Velocity)>) {
    for (psc, grounded, mut vel) in q.iter_mut() {
        match grounded.0 {
            GroundedState::NotGrounded => {
                vel.0.x = vel.0.x.clamp(-psc.air_top_speed_x, psc.air_top_speed_x);
                vel.0.y = vel.0.y.clamp(-psc.air_top_speed_y, psc.air_top_speed_y);
            }
            GroundedState::GroundedTo(_) => {
                vel.0.x = vel.0.x.clamp(-psc.ground_top_speed, psc.ground_top_speed);
            }
        }
    }
}

/// Friction is applied if the player is not being moved, or trying to move in the opposite direction
/// of the velocity, and the animation is a drifting animation, and the velocity is below a certain threshold
fn friction(
    mut q: Query<(
        &RiderSpeedCharacteristics,
        &Velocity,
        &MovementControl,
        &mut Force,
        &RiderAnimationState,
    )>,
) {
    let friction_constant = 40.0;
    for (psc, vel, mc, mut force, ras) in q.iter_mut() {
        match ras {
            RiderAnimationState::Drifting => {
                if mc.0 .0.x * vel.0.x < 0.0 || mc.0 .0.x == 0.0 {
                    let sign;
                    if vel.0.x > 0.0 {
                        sign = 1.0;
                    } else {
                        sign = -1.0;
                    }
                    force.0.x += -1.0 * sign * vel.0.x.abs().clamp(1.0, 10.0) * friction_constant;
                }
            }
            _ => {}
        }
    }
}
