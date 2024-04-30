use bevy::{app::PluginGroupBuilder, prelude::*};
use eggman_control::EggmanControlPlugin;
use movement_control::MovementControlPlugin;
use pter_control::PterControlPlugin;

pub mod eggman_control;
pub mod movement_control;
pub mod pter_control;

pub struct BehaviorPlugins;
impl PluginGroup for BehaviorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(eggman_control::EggmanControlPlugin)
            .add(PterControlPlugin)
            .add(EggmanControlPlugin)
            .add(MovementControlPlugin)
    }
}
