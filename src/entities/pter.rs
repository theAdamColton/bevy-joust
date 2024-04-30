use crate::{entities::spritesheets::*, engine::{collision::SquareCollider, physics::{Mass, Position, Velocity}}, constants::GLOBAL_SPRITE_SCALE, behavior::{movement_control::MovementControl, pter_control::PterControlBundle}};
use crate::engine::collision::ColliderBundle;
use crate::engine::physics::{V2, PhysicsBodyBundle};
use crate::animation::pter_animation::PterAnimationBundle;

use bevy::prelude::*;

pub struct PterPlugin;
impl Plugin for PterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PterSpawnEvent>();
        app.add_systems(Update, pter_spawn_event_listener);
    }
}

#[derive(Event)]
pub struct PterSpawnEvent(pub PterSpawnEventDetails);
pub struct PterSpawnEventDetails {
    pub pos: V2,
    pub vel: V2,
}

#[derive(Bundle)]
pub struct PterBundle {
    pub sprite: SpriteSheetBundle,
    pub pter: PterSprite,
    pub coll: ColliderBundle,
    pub phys_b: PhysicsBodyBundle,
    pub pc: PterControlBundle,
    pub pab: PterAnimationBundle,
}

#[derive(Component)]
pub struct PterSprite;

fn pter_spawn_event_listener(
    mut commands: Commands,
    mut events: EventReader<PterSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<PterTextureAtlas>>
) {
    for e in events.iter() {

        let coll = ColliderBundle {
            sq: SquareCollider {
                min: V2::new(-2.0, -0.5),
                max: V2::new(2.0, 0.5),
                ..Default::default()
            },
            ..Default::default()
        };

        let pb = PhysicsBodyBundle {
            m: Mass(20.0),
            p: Position(e.0.pos),
            v: Velocity(e.0.vel),
            ..Default::default()
        };

        let ssb = SpriteSheetBundle {
            texture_atlas: q.single().clone(),
            transform: Transform {
                scale: Vec3::splat(2.5 * GLOBAL_SPRITE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        };

        let pb = PterBundle {
            sprite: ssb,
            pter: PterSprite,
            coll,
            phys_b: pb,
            pc: PterControlBundle::default(),
            pab: PterAnimationBundle::default(),
        };
        commands.spawn(pb);
    }
}
