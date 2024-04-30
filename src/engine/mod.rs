pub mod collision;
pub mod physics;
pub mod wraparound;
pub mod speed_clamps;
pub mod despawn;

use collision::CollisionPlugin;
use physics::PhysicsPlugin;
use wraparound::WraparoundPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct DefaultEnginePlugins;
impl PluginGroup for DefaultEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
         .add(CollisionPlugin)
        .add(PhysicsPlugin)
        .add(WraparoundPlugin)
        .add(speed_clamps::SpeedClampPlugin)
        .add(despawn::DespawnPlugin)
    }
}
