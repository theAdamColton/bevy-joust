/*
Controls the lifecycle and movement of the eggman

Lifecycle is as follows:
    Egg: Eggman doesn't move
    Hatching: Eggman still doesn't move
    Hatched: Eggman still doesn't move
    Seeking: Eggman finally will start to move around

Each section of the lifecycle is timed, as determined by each eggman's
EggmanLifecycleCharacteristics component.
*/

use crate::behavior::movement_control::MovementControl;
use crate::engine::collision::{NonStaticCollisionEvent, ShouldCalculateNonStaticIntersectionsOn};
use crate::engine::physics::Position;
use crate::engine::speed_clamps::SpeedClamps;
use crate::entities::eggman::Eggman;
use crate::player::player_control::PlayerController;
use bevy::prelude::*;
use cgmath::InnerSpace;
use std::time::Duration;

#[derive(Bundle)]
pub struct EggmanControlBundle {
    mc: MovementControl,
    es: EggmanState,
    bt: BehaviorTimer,
    ebc: EggmanLifecycleCharacteristics,
    esc: EggmanSpeedCharacteristics,
    speed_clamps: SpeedClamps,
    should: ShouldCalculateNonStaticIntersectionsOn,
}
impl Default for EggmanControlBundle {
    fn default() -> Self {
        Self {
            mc: MovementControl::default(),
            es: EggmanState::JustSpawned,
            bt: BehaviorTimer(Timer::from_seconds(10.0, TimerMode::Once)),
            ebc: EggmanLifecycleCharacteristics::default(),
            esc: EggmanSpeedCharacteristics::default(),
            speed_clamps: SpeedClamps::new_from_x_y(3.0, 10.0),
            should: ShouldCalculateNonStaticIntersectionsOn,
        }
    }
}

/// Controls characterisitcs of this eggman's lifecycle
/// in seconds
#[derive(Component)]
pub struct EggmanLifecycleCharacteristics {
    pub incubation_period: f32,
    pub hatch_period_time: f32,
    pub hatched_period_time: f32,
}
impl Default for EggmanLifecycleCharacteristics {
    fn default() -> Self {
        Self {
            incubation_period: 5.0,
            hatch_period_time: 10.0,
            hatched_period_time: 2.0,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct BehaviorTimer(Timer);

#[derive(Component)]
pub enum EggmanState {
    JustSpawned,
    Egg,
    Hatching,
    Hatched,
    /// The Eggman is going after the player
    Seeking,
}

#[derive(Component)]
pub struct DeadEggman;

pub struct EggmanControlPlugin;
impl Plugin for EggmanControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(advance_lifecycle, control_eggman));
    }
}

fn advance_lifecycle(
    mut q: Query<(
        &mut BehaviorTimer,
        &mut EggmanState,
        &EggmanLifecycleCharacteristics,
    )>,
    time: Res<Time>,
) {
    use EggmanState::*;

    for (mut bt, mut es, elc) in q.iter_mut() {
        bt.tick(time.delta());
        match es.as_ref() {
            JustSpawned => {
                bt.set_duration(Duration::from_secs_f32(elc.incubation_period));
                bt.reset();
                *es = Egg;
                println!("Spawned");
            }
            Egg => {
                if bt.finished() {
                    // Starts Hatching this egg this frame
                    *es = Hatching;
                    bt.set_duration(Duration::from_secs_f32(elc.hatch_period_time));
                    bt.reset();
                    println!("Hatching");
                }
            }
            Hatching => {
                if bt.finished() {
                    *es = Hatched;
                    bt.set_duration(Duration::from_secs_f32(elc.hatched_period_time));
                    bt.reset();
                    println!("Hatched!");
                }
            }
            Hatched => {
                if bt.finished() {
                    *es = Seeking;
                    println!("Seeking");
                }
            }
            Seeking => {}
        }
    }
}

#[derive(Component)]
pub struct EggmanSpeedCharacteristics {
    pub hor_accel: f32,
    pub vert_accel: f32,
}
impl Default for EggmanSpeedCharacteristics {
    fn default() -> Self {
        Self {
            hor_accel: 10.0,
            vert_accel: 0.0,
        }
    }
}

fn control_eggman(
    mut q: Query<(
        &mut MovementControl,
        &Position,
        &EggmanSpeedCharacteristics,
        &EggmanState,
    )>,
    q_player: Query<&Position, With<PlayerController>>,
) {
    // TODO Only works for a single player.
    if let Ok(player_pos) = q_player.get_single() {
        for (mut mc, pos, esc, es) in q.iter_mut() {
            // Only controls movement if this eggman is in the seeking state
            match es {
                EggmanState::Seeking => {
                    let direction = (player_pos.0 - pos.0).normalize();

                    mc.0 .0.x = direction.x * esc.hor_accel;
                }
                _ => {}
            }
        }
    }
}

fn handle_collision(
    mut commands: Commands,
    q: Query<(Entity, &EggmanState, &NonStaticCollisionEvent)>,
    q_player: Query<&Position, With<PlayerController>>,
) {
    for (ent, eggman_state, coll_event) in q.iter() {
        //commands.entity(ent).remove::<NonStaticCollisionEvent>();

        match eggman_state {
            EggmanState::Hatched => {}
            EggmanState::Seeking => {}
            _ => {
                // Eggman is in a crushable state
                match q_player.get(coll_event.0) {
                    Ok(pos) => {
                        // Eggman is colliding with the player,
                        // Eggman gets crushed
                        commands.entity(ent).insert(DeadEggman);
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
