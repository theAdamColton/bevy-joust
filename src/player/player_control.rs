use crate::engine::collision::{Grounded, GroundedState};
use crate::behavior::movement_control::MovementControl;
use crate::engine::physics::V2;
use crate::entities::rider_physics::RiderSpeedCharacteristics;
use bevy::prelude::*;

pub struct PlayerControlPlugin;
impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,apply_input);
    }
}

#[derive(Bundle, Copy, Clone)]
pub struct PlayerControllerBundle {
    pc: PlayerController,
    mc: MovementControl,
    psc: RiderSpeedCharacteristics,
}
impl Default for PlayerControllerBundle {
    fn default() -> Self {
        Self {
            pc: PlayerController { input_source: InputSource::Wasd },
            mc: MovementControl::default(),
            psc: RiderSpeedCharacteristics::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum InputSource {
    Wasd,
}

#[derive(Component, Copy, Clone)]
pub struct PlayerController {
    pub input_source: InputSource,
}


fn apply_input(
    mut q: Query<(
        &PlayerController,
        &mut MovementControl,
        &RiderSpeedCharacteristics,
        &Grounded,
    )>,
    key_in: Res<Input<KeyCode>>,
) {
    for (pc, mut mc, psc, grounded) in q.iter_mut() {
        let mut movement = V2::new(0.0, 0.0);
        match pc.input_source {
            InputSource::Wasd => {
                if key_in.pressed(KeyCode::A) {
                    movement.x = -1.0;
                }
                if key_in.pressed(KeyCode::D) {
                    movement.x = 1.0;
                }
                if key_in.just_pressed(KeyCode::Z) {
                    movement.y = 1.0;
                }
                if key_in.just_pressed(KeyCode::X) {
                    movement.y = 1.0;
                }
                if key_in.just_pressed(KeyCode::W) {
                    movement.y = 1.0;
                }
            }
        }

        apply_movement(&movement, mc, psc, grounded);
    }
}

fn apply_movement(
    movement: &V2,
    mut mc: Mut<MovementControl>,
    psc: &RiderSpeedCharacteristics,
    grounded: &Grounded,
) {
    match grounded.0 {
        GroundedState::NotGrounded => {
            mc.0.0 = V2::new(movement.x * psc.air_force, movement.y * psc.jump_acceleration_from_air);
        },
        GroundedState::GroundedTo(_) => {
            mc.0.0 = V2::new(movement.x * psc.ground_acceleration, movement.y * psc.jump_acceleration_from_ground);
        },
    }
}
