use crate::animation::{AnimationStages, AnimationTimer, };
use crate::engine::physics::Velocity;
use bevy::prelude::*;

pub struct PterAnimationPlugin;
impl Plugin for PterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (compute_next_frame
                .in_set(AnimationStages::CalculateNextStates)
                .before(AnimationStages::ApplyAnimationState),
                apply_frame.in_set(AnimationStages::ApplyAnimationState)
            )
        );
    }
}

#[derive(Component, DerefMut, Deref)]
struct BeginFlapTimer(AnimationTimer);
#[derive(Component, DerefMut, Deref)]
struct EndFlapTimer(AnimationTimer);

#[derive(Bundle)]
pub struct PterAnimationBundle {
    bft: BeginFlapTimer,
    eft: EndFlapTimer,
    state: PterAnimationState,
}
impl Default for PterAnimationBundle {
    fn default() -> Self {
        Self {
            bft: BeginFlapTimer(AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))),
            eft: EndFlapTimer(AnimationTimer(Timer::from_seconds(0.5, TimerMode::Once))),
            state: PterAnimationState::Coast,
        }
    }
}

#[derive(Component)]
pub enum PterAnimationState {
    /// Wings down
    Downflap,
    /// Wings up
    Upflap,
    /// Wings middle
    Coast,
}
impl PterAnimationState {
    fn get_frame(&self) -> usize {
        use PterAnimationState::*;
        match &self {
            Downflap => 2,
            Upflap => 0,
            Coast => 1,
        }
    }

    fn from_usize(i: usize) -> Self {
        use PterAnimationState::*;
        match i {
            2 => Downflap,
            0 => Upflap,
            1 => Coast,
            _ => panic!(),
        }
    }
}

impl Into<PterAnimationState> for usize {
    fn into(self) -> PterAnimationState {
        PterAnimationState::from_usize(self)
    }
}

fn compute_next_frame(
    mut q: Query<(
        &mut PterAnimationState,
        &Velocity,
        &mut BeginFlapTimer,
        &mut EndFlapTimer,
    )>,
    time: Res<Time>,
) {
    use PterAnimationState::*;

    // Threshold for raising wings
    let upflap_thresh = 0.1;
    let downflap_thresh = 0.1;

    for (mut pas, vel, mut bft, mut eft) in q.iter_mut() {
        bft.tick(time.delta());
        eft.tick(time.delta());

        match pas.as_ref() {
            Downflap => {
                if eft.finished() {
                    *pas = Coast;
                    continue;
                }
            }
            _ => {}
        }
        if vel.0.y > downflap_thresh {
            // Only enters a downflap if the timer is done
            if bft.finished() {
                *pas = Downflap;
                bft.reset();
                eft.reset();
            }
        } else if vel.0.y < -upflap_thresh {
            *pas = Upflap;
        } else {
            *pas = Coast;
        }
    }
}

fn apply_frame(mut q: Query<(&mut TextureAtlasSprite, &PterAnimationState, &Velocity)>) {
    for (mut tas, pas, vel) in q.iter_mut() {
        if vel.0.x > 0.1 {
            tas.flip_x = false;
        } else if vel.0.x < -0.1 {
            tas.flip_x = true;
        }
        tas.index = pas.get_frame();
    }
}
