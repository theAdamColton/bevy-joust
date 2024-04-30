pub mod eggman_animation;
pub mod pter_animation;
pub mod rider_animation;
mod pop;

use crate::engine::physics::Velocity;
use crate::engine::physics::{PhysicsStages};
use bevy::prelude::*;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        use AnimationStages::*;
        app.add_systems(Update, 
            (
                tick_animation_timers.in_set(CalculateNextStates),
                (apply_frames, flip_sprites).in_set(ApplyAnimationState)
            ),
        );
        app.configure_sets(Update, (
                // This inclusion hopefully stops the rider from being in the 'falling' state when
                // on the ground
                CalculateNextStates.before(PhysicsStages::CalculateNextVelocities),
                ApplyAnimationState
        ).chain());

        app.add_plugins(
            (pter_animation::PterAnimationPlugin,
            rider_animation::RiderAnimationPlugin,
            eggman_animation::EggmanAnimationPlugin,
            pop::PopAnimationPlugin,)
            );
    }
}

/// Insert this component into an entity with a TextureAtlasSprite to get it to animate on the
/// ApplyAnimationState state
#[derive(Component, Deref, DerefMut)]
pub struct Frame(usize);

/// Insert this component into an entity with a TextureAtlasSprite and Velocity to get it to flip
/// every frame
#[derive(Component)]
pub struct FlipSpriteSheetBasedOnVelocity;

/// If you want to manually set the frame of an animatable,
/// set it between the CalculateNextStates and ApplyAnimationState
#[derive(SystemSet, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum AnimationStages {
    /// During this stage, index of the next frame is calculated
    CalculateNextStates,
    /// During this stage, the index is applied to the TextureAtlasSprite
    ApplyAnimationState,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn tick_animation_timers(mut q: Query<&mut AnimationTimer>, time: Res<Time>) {
    for mut timer in q.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn apply_frames(mut q: Query<(&Frame, &mut TextureAtlasSprite)>) {
    for (f, mut tas) in q.iter_mut() {
        tas.index = **f;
    }
}

fn flip_sprites(
    mut q: Query<(&mut TextureAtlasSprite, &Velocity), With<FlipSpriteSheetBasedOnVelocity>>,
) {
    for (mut tas, vel) in q.iter_mut() {
        if vel.0.x > 0.1 {
            tas.flip_x = false;
        } else if vel.0.x < -0.1 {
            tas.flip_x = true;
        }
    }
}
