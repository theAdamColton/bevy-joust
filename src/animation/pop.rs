use bevy::prelude::*;
use crate::animation::{AnimationTimer, Frame, AnimationStages};
use crate::entities::spritesheets::PopTextureAtlas;

pub struct PopAnimationPlugin;
impl Plugin for PopAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_pop.in_set(AnimationStages::CalculateNextStates));
    }
}

#[derive(Component)]
pub struct Pop;

/// Insert this bundle to get a pop animation to appear in the world
#[derive(Bundle)]
pub struct PopAnimationBundle {
    pub timer: AnimationTimer,
    pub frame: Frame,
    pub pop: Pop,
}
impl Default for PopAnimationBundle {
    fn default() -> Self {
        Self {
            timer: AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            frame: Frame(0),
            pop: Pop,
        }
    }
}

fn animate_pop(mut q: Query<(&mut Frame, &AnimationTimer), With<Pop>>) {
    for (mut f, at) in q.iter_mut() {
        if at.finished() {
            f.0 += 1;
            if f.0 == 9 {
                f.0 = 0;
            }
        }
    }
}
