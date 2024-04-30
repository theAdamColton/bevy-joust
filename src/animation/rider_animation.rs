use crate::animation::{AnimationStages, AnimationTimer};
use crate::behavior::movement_control::MovementControl;
use crate::engine::collision::{Grounded, GroundedState};
use crate::engine::physics::{Acceleration, Velocity, };
use bevy::prelude::*;
use std::time::Duration;

const ANIMATION_DURATION: f32 = 100.0 / 1000.0;

pub struct RiderAnimationPlugin;
impl Plugin for RiderAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((compute_next_frame)
                .in_set(AnimationStages::CalculateNextStates),
                apply_frame.in_set(AnimationStages::ApplyAnimationState)
            )
        );
    }
}

#[derive(Bundle)]
pub struct RiderAnimationBundle {
    pub timer: AnimationTimer,
    state: RiderAnimationState,
}
impl Default for RiderAnimationBundle {
    fn default() -> Self {
        Self {
            timer: AnimationTimer(Timer::from_seconds(ANIMATION_DURATION, TimerMode::Repeating)),
            state: RiderAnimationState::Walking(0),
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum RiderAnimationState {
    Downflap,
    /// Latent state when not touching ground
    Upflap,
    /// Indicates the walk frame, from 0 to 3
    Walking(usize),
    Standing,
    Drifting,
}
impl RiderAnimationState {
    fn get_frame(&self) -> usize {
        use RiderAnimationState::*;
        match &self {
            Downflap => 5,
            Upflap => 6,
            Standing => 3,
            Walking(x) => *x,
            Drifting => 4,
        }
    }

    fn from_usize(i: usize) -> Self {
        use RiderAnimationState::*;
        match i {
            0..=2 => Walking(i),
            3 => Standing,
            4 => Drifting,
            5 => Downflap,
            6 => Upflap,
            _ => panic!(),
        }
    }
}

impl Into<RiderAnimationState> for usize {
    fn into(self) -> RiderAnimationState {
        RiderAnimationState::from_usize(self)
    }
}

fn compute_next_frame(
    mut q: Query<(
        &mut RiderAnimationState,
        &Velocity,
        &Grounded,
        &mut AnimationTimer,
        &Acceleration,
    )>,
) {
    for (ras, vel, grounded, timer, acc) in q.iter_mut() {
        match grounded.0 {
            GroundedState::GroundedTo(_) => {
                change_state_from_grounded(ras, vel, timer, acc);
            }
            GroundedState::NotGrounded => {
                change_state_from_air(ras, timer, acc);
            }
        }
    }
}

fn apply_frame(
    mut q: Query<(
        &mut TextureAtlasSprite,
        &RiderAnimationState,
        &Velocity,
        &MovementControl,
    )>,
) {
    for (mut tas, ras, vel, mc) in q.iter_mut() {
        tas.index = ras.get_frame();
        match ras {
            /*
            Flips the sprite sheet based on velocity if not drifting,
            if drifting, the spritesheet is flipped by movement control
            */
            RiderAnimationState::Drifting => {
                if vel.0.x > 0.1 {
                    tas.flip_x = false;
                } else if vel.0.x < -0.1 {
                    tas.flip_x = true;
                }
            }
            _ => {
                if mc.0 .0.x > 0.1 {
                    tas.flip_x = false;
                } else if mc.0 .0.x < -0.1 {
                    tas.flip_x = true;
                }
            }
        }
    }
}

/// Goes from any animation into the grounded animations
fn change_state_from_grounded(
    mut ras: Mut<RiderAnimationState>,
    vel: &Velocity,
    mut atimer: Mut<AnimationTimer>,
    acc: &Acceleration,
) {
    use RiderAnimationState::*;

    // Top speed that the rider will enter the falling animation
    let vertical_stop_thresh = 0.2;
    // Top speed that rider will be able to enter drifting animation
    let drift_speed_thresh = 0.01;
    // acc * speed must be less than negative of this number to start drifting
    let drift_acc_thresh = 0.01;
    // Exits walking from below this speed threshold
    let exit_walk_speed_thresh = 0.05;

    if vel.0.y.abs() > vertical_stop_thresh {
        *ras = Upflap;
        reset_and_pause_timer(atimer);
    } else if ras.get_frame() == Drifting.get_frame() {
        /*
         * If the Rider was in a drifting animation,
         * it will exit it if the x velocity and acceleration are
         * in the same direction, or if horizontal velocity is low enough
         * TODO adjust exit velocity
         */
        if vel.0.x.abs() < drift_speed_thresh {
            *ras = Standing;
        } else if acc.0.x * vel.0.x > drift_speed_thresh {
            *ras = Walking(0);
            reset_timer_for_walking(atimer, vel.0.x);
        }
    } else if acc.0.x * vel.0.x < -drift_acc_thresh && vel.0.x.abs() > drift_speed_thresh {
        /*
         * If x acceleration and x velocity are in opposite directions,
         * and the velocity is above a certain point
         * The rider will enter the drift animation.
         */
        *ras = Drifting;
        reset_and_pause_timer(atimer);
    } else if let Walking(x) = ras.as_ref() {
        /*
         * If already Grounded and walking, the sprite will continue,
         * unless it's velocity is below a certain magnitude
         */
        if vel.0.x.abs() < exit_walk_speed_thresh {
            *ras = Standing;
            reset_and_pause_timer(atimer);
        } else if atimer.0.just_finished() {
            // Iterates the walking frame
            *ras = ((x + 1) % 3).into();
            atimer.0.set_duration(Duration::from_secs_f32(
                ANIMATION_DURATION * 5.0 / vel.0.x.abs(),
            ));
        }
    } else {
        // Top speed that the rider will enter the walking animation
        // from a standing animation
        let horizontal_stop_thresh = 0.1;
        if vel.0.x.abs() > horizontal_stop_thresh {
            *ras = Walking(0);
            reset_timer_for_walking(atimer, vel.0.x);
        } else {
            *ras = Standing;
            reset_and_pause_timer(atimer);
        }
    }
}

fn change_state_from_air(
    mut ras: Mut<RiderAnimationState>,
    atimer: Mut<AnimationTimer>,
    acc: &Acceleration,
) {
    use RiderAnimationState::*;
    if ras.get_frame() == Downflap.get_frame() {
        if atimer.0.just_finished() {
            *ras = Upflap;
            reset_and_pause_timer(atimer);
        }
    } else {
        if acc.0.y > 0.05 {
            reset_timer_for_flapping(atimer);
            *ras = Downflap;
        } else {
            *ras = Upflap;
        }
    }
}

fn reset_timer_for_walking(mut atimer: Mut<AnimationTimer>, mut speed: f32) {
    speed = speed.abs();
    atimer
        .0
        .set_duration(Duration::from_secs_f32(ANIMATION_DURATION));
    atimer.0.reset();
    atimer.0.unpause();
}

fn reset_and_pause_timer(mut atimer: Mut<AnimationTimer>) {
    atimer.0.pause();
    atimer.0.reset();
}

fn reset_timer_for_flapping(mut atimer: Mut<AnimationTimer>) {
    atimer
        .0
        .set_duration(Duration::from_secs_f32(ANIMATION_DURATION * 0.8));
    atimer.0.unpause();
}
