pub mod platform;
pub mod pter;
pub mod rider;
pub mod rider_physics;
pub mod spritesheets;
pub mod eggman;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct JoustEntitiesPlugins;
impl PluginGroup for JoustEntitiesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(pter::PterPlugin)
            .add(eggman::EggmanPlugin)
            .add(rider::RiderPlugin)
            .add(rider_physics::RiderPhysicsPlugin)
            .add(spritesheets::JoustSpriteSheetPlugin)
            .add(platform::PlatformPlugin)
    }
}
