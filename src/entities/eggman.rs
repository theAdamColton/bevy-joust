use crate::constants::{GLOBAL_COLLIDER_SCALE, GLOBAL_SPRITE_SCALE};
use crate::engine::collision::{ColliderBundle, SquareCollider};
use crate::engine::physics::V2;
use crate::engine::physics::{Mass, PhysicsBodyBundle, Position, Velocity};
use crate::animation::eggman_animation::EggmanAnimationBundle;
use crate::entities::spritesheets::EggTextureAtlas;
use crate::behavior::eggman_control::EggmanControlBundle;
use bevy::prelude::*;

pub struct EggmanPlugin;
impl Plugin for EggmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EggmanSpawnEvent>();
        app.add_systems(Update,spawn_eggman_listener);
    }
}

#[derive(Component)]
pub struct Eggman;

#[derive(Bundle)]
pub struct EggmanBundle {
    ssb: SpriteSheetBundle,
    eab: EggmanAnimationBundle,
    eggman: Eggman,
    coll: ColliderBundle,
    phys_b: PhysicsBodyBundle,
    ecb: EggmanControlBundle,
}

#[derive(Event)]
pub struct EggmanSpawnEvent {
    pub position: V2,
    pub velocity: V2,
}
impl Default for EggmanSpawnEvent {
    fn default() -> Self {
        Self {
            position: V2::new(0.0, 0.0),
            velocity: V2::new(0.0, 0.0),
        }
    }
}

fn spawn_eggman_listener(
    mut commands: Commands,
    mut events: EventReader<EggmanSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<EggTextureAtlas>>,
) {
    for e in events.read() {
        let ssb = SpriteSheetBundle {
            texture_atlas: q.single().clone(),
            transform: Transform {
                scale: Vec3::splat(2.5 * GLOBAL_SPRITE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        };

        let pb = PhysicsBodyBundle {
            m: Mass(5.0),
            p: Position(e.position),
            v: Velocity(e.velocity),
            ..Default::default()
        };

        let coll = ColliderBundle {
            sq: SquareCollider {
                min: V2::new(-0.5, -0.5),
                max: V2::new(0.5, 0.5),
                offset: V2::new(0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        };

        let eab = EggmanAnimationBundle::default();

        let eggmanbundle = EggmanBundle {
            eab,
            ssb,
            eggman: Eggman,
            coll,
            phys_b: pb,
            ecb: EggmanControlBundle::default(),
        };

        commands.spawn(eggmanbundle);
    }
}
