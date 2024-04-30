pub mod player_control;
pub mod player_damage;

use player_control::PlayerControlPlugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::engine::collision::ShouldCalculateNonStaticIntersectionsOn;

use self::player_control::PlayerControllerBundle;

#[derive(Component)]
pub struct Player;

/// Insert this into a rider to control it as a player
#[derive(Bundle, Copy, Clone)]
pub struct PlayerBundle {
    pcb: PlayerControllerBundle,
    intersections: ShouldCalculateNonStaticIntersectionsOn,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            pcb: PlayerControllerBundle::default(),
            intersections: ShouldCalculateNonStaticIntersectionsOn,
        }
    }
}

pub struct PlayerPluginGroup;
impl PluginGroup for PlayerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerControlPlugin)
    }
}
