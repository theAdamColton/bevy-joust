use crate::animation::{AnimationStages, AnimationTimer, FlipSpriteSheetBasedOnVelocity, Frame};
use crate::animation::pop::PopAnimationBundle;
use crate::behavior::eggman_control::{DeadEggman, EggmanLifecycleCharacteristics, EggmanState};
use crate::engine::physics::Velocity;
use crate::entities::eggman::Eggman;
use crate::entities::spritesheets::{EggTextureAtlas, PopTextureAtlas};
use bevy::prelude::*;

pub struct EggmanAnimationPlugin;
impl Plugin for EggmanAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (compute_next_frame, animate_dead_eggmen).in_set(AnimationStages::CalculateNextStates));
    }
}

#[derive(Bundle)]
pub struct EggmanAnimationBundle {
    pub timer: AnimationTimer,
    pub frame: Frame,
    flip: FlipSpriteSheetBasedOnVelocity,
}
impl Default for EggmanAnimationBundle {
    fn default() -> Self {
        let animation_duration = 0.8;
        Self {
            timer: AnimationTimer(Timer::from_seconds(animation_duration, TimerMode::Once)),
            frame: Frame(0),
            flip: FlipSpriteSheetBasedOnVelocity,
        }
    }
}

fn compute_next_frame(
    mut q: Query<(
        &EggmanState,
        &EggmanLifecycleCharacteristics,
        &mut Frame,
        &mut AnimationTimer,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for (es, elc, mut frame, mut at, vel) in q.iter_mut() {
        use EggmanState::*;

        match es {
            JustSpawned => {}
            Egg => {
                // No animation occurs
            }
            Hatching => {
                // Flips between the first and the second frame
                at.tick(time.delta());

                if at.finished() {
                    **frame = (frame.0 + 1) % 2;
                    at.reset()
                }
            }
            Hatched => {
                if frame.0 > 4 {
                    break;
                }
                // Flips from the third to the fifth frame
                at.tick(time.delta());

                if at.finished() {
                    **frame = frame.0 + 1;
                    at.reset()
                }
            }
            Seeking => {
                at.tick(time.delta());

                if at.finished() {
                    let animation_speed_thresh = 0.2;
                    if vel.x > animation_speed_thresh {
                        **frame = frame.0 % 2 + 5;
                        at.reset()
                    } else {
                        **frame = 5;
                    }
                }
            }
        }
    }
}

/// The Eggman texture is replaced with the pop texture for newly deceased eggmen
fn animate_dead_eggmen(
    mut c: Commands,
    q: Query<Entity, Added<DeadEggman>>,
    q_pop_tex: Query<&Handle<TextureAtlas>, With<PopTextureAtlas>>,
) {
    for dead_ent in q.iter() {
        let pop_tex = q_pop_tex.single().clone();
        let pop_animation_bundle = PopAnimationBundle::default();
        let mut ent = c.entity(dead_ent);
        ent.remove::<EggTextureAtlas>();
        ent.insert(pop_tex);
        ent.insert(pop_animation_bundle);
    }
}
