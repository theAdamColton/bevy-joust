/*
Steers this Pter towards the Player controlled rider
*/
use std::time::Duration;

use crate::{engine::physics::{Position, Velocity, V2}, entities::pter::*, player::player_control::PlayerController};
use crate::engine::speed_clamps::SpeedClamps;

use bevy::prelude::*;
use cgmath::InnerSpace;

use super::movement_control::MovementControl;

#[derive(Bundle)]
pub struct PterControlBundle {
    pub mc: MovementControl,
    dt: DiveTimer,
    ft: FlapTimer,
    pcms: PterControlMovementState,
    psc: PterSpeedCharacteristics,
    speed_clamps: SpeedClamps,
}
impl Default for PterControlBundle {
    fn default() -> Self {
        let max_horiz_speed = 10.0;
        let max_vert_speed = 10.0;
        Self {
            mc: MovementControl::default(),
            dt: DiveTimer(Timer::new(Duration::from_secs_f32(8.0), TimerMode::Once)),
            ft: FlapTimer(Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once)),
            pcms: PterControlMovementState::default(),
            psc: PterSpeedCharacteristics::default(),
            speed_clamps: SpeedClamps{pos_x: max_horiz_speed, neg_x: -max_horiz_speed, pos_y: max_vert_speed, neg_y: -1.0E20},
        }
    }
}

pub struct PterControlPlugin;
impl Plugin for PterControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,control_pter);
    }
}

#[derive(Component)]
pub enum PterControlMovementState {
    Diving,
    Flapping,
    Coasting,
}
impl Default for PterControlMovementState {
    fn default() -> Self {
        Self::Coasting
    }
}

#[derive(Component)]
pub struct PterSpeedCharacteristics {
    dive_force: f32,
    flap_force: f32,
}
impl Default for PterSpeedCharacteristics {
    fn default() -> Self {
        Self {
            dive_force: 4.0,
            flap_force: 10.0,
        }
    }
}

#[derive(Component)]
struct DiveTimer(Timer);
#[derive(Component)]
struct FlapTimer(Timer);

fn control_pter(
    mut q: Query<(&mut MovementControl, &Position, &PterSpeedCharacteristics, &mut PterControlMovementState, &mut DiveTimer, &mut FlapTimer), With<PterSprite>>,
    q_player: Query<(&Position), With<PlayerController>>,
    time: Res<Time>,
) {
    // TODO only works with single player
    if let Ok(player_pos) = q_player.get_single() {
        for (mut mc, pos, psc, mut pcms, mut dt, mut ft) in q.iter_mut() {
            dt.0.tick(time.delta());
            ft.0.tick(time.delta());
            // Direction to player
            let direction = (player_pos.0 - pos.0).normalize();

            // Should the pter dive? If it is below and the timer is up
            if dt.0.finished() && direction.y < 0.0 {
                *pcms = PterControlMovementState::Diving;
                dt.0.reset();
                mc.0.0.y = direction.y * 3.0;
                mc.0.0.x = direction.x * 1.0;
            }

            // Should the pter flap? If the player is above and the timer is up
            else if ft.0.finished() {
                *pcms = PterControlMovementState::Flapping;
                ft.0.reset();
                mc.0.0.y = psc.flap_force;
            } else {
                *pcms = PterControlMovementState::Coasting;
            }
        }
    }
}

