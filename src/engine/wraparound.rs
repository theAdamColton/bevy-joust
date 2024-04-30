/*
    Every Entity with a Position and without a ShouldntWraparound component
will exhibit wraparound behavior.

    A system continually checks the positions of all entities,
all positions are constrained to -1/2 BorderDistance to +1/2 BorderDistance

    Also, when an entity with a texture atlas handle and a position
gets within a certain distance to the border, a ghost sprite is spawned
on the other side of the screen. This sprite is removed after leaving a
certain distance from the border.
*/

use crate::{
    constants::*,
    engine::collision::ColliderBundle,
    engine::physics::{PhysicsBodyBundle, Velocity},
};
use bevy::prelude::*;
use modulo::Mod;

use crate::{
    engine::collision::SquareCollider,
    engine::physics::{PhysicsScale, PhysicsStages, Position, V2},
};

pub struct WraparoundPlugin;
impl Plugin for WraparoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BorderDistance(48.0));
        app.add_systems(
            Update,
            modulo_position_wraparound
                .after(PhysicsStages::CalculateNextPositions)
                .before(PhysicsStages::CalculateNextForces),
        );
        app.add_systems(Update,start_wraparound);
        app.add_systems(Update,stop_wraparound);
        app.add_systems(Update,wraparound_ghost_position_sync);
        app.add_systems(Update,wraparound_ghost_texture_sync);
    }
}

/// Implements wraparound position syncing
/// Every position will become between - BorderDistance / 2 and + BorderDistance / 2
fn modulo_position_wraparound(
    border_distance: Res<BorderDistance>,
    mut q: Query<&mut Position, Without<ShouldntWraparound>>,
) {
    for mut pos in q.iter_mut() {
        pos.0.x = coord_space_to_wraparound_space(pos.0.x, border_distance.0);
    }
}

pub fn coord_space_to_wraparound_space(x: f32, border_distance: f32) -> f32 {
    (x + border_distance / 2.0).modulo(border_distance) - border_distance / 2.0
}

/// This component indicates that this entity should not have it's position
/// contrained to between -borderdistance / 2 and + borderdistance /2
#[derive(Component)]
pub struct ShouldntWraparound;

#[derive(Component)]
pub struct IsWrapped;

            
#[derive(Deref, DerefMut, Resource)]
pub struct BorderDistance(pub f32);

/// A wraparound ghost is inserted into entities with a TextureAtlasSprite and a
/// SquareCollider. The translation of the ghost is synced to the parent translation - BorderDistance
#[derive(Bundle)]
struct WraparoundGhostBundle {
    pos: Position,
    ssb: SpriteSheetBundle,
    ghost: WraparoundGhost,
    sw: ShouldntWraparound,
}

#[derive(Component)]
pub struct WraparoundGhost {
    pub parent: Entity,
    pub offset: f32,
}

const GHOST_WRAPAROUND_THRESH: f32 = 6.0;

fn start_wraparound(
    mut commands: Commands,
    q: Query<
        (Entity, &Handle<TextureAtlas>, &Position, &Transform),
        (
            Without<WraparoundGhost>,
            Without<ShouldntWraparound>,
            Without<IsWrapped>,
        ),
    >,
    bd: Res<BorderDistance>,
    ps: Res<PhysicsScale>,
) {
    for (ent, tas, pos, t) in q.iter() {
        if pos.0.x < -bd.0 / 2.0 + GHOST_WRAPAROUND_THRESH || pos.0.x > bd.0 / 2.0 - GHOST_WRAPAROUND_THRESH {
            let offset: f32;
            if pos.0.x > 0.0 {
                offset = -bd.0;
            } else {
                offset = bd.0;
            }

            let ghost_pos = Position(V2::new(pos.0.x + offset, pos.0.y));

            let ghost_ssb = SpriteSheetBundle {
                texture_atlas: tas.clone(),
                transform: Transform {
                    scale: t.scale,
                    translation: Vec3::new(
                        ps.0 * t.translation.x,
                        t.translation.y,
                        t.translation.z,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            };
            let ghost = WraparoundGhostBundle {
                pos: ghost_pos,
                ssb: ghost_ssb,
                ghost: WraparoundGhost {
                    parent: ent,
                    offset,
                },
                sw: ShouldntWraparound,
            };

            commands.entity(ent).insert(IsWrapped);
            commands.spawn(ghost);
            println!("Spawned new wraparound ghost.");
        }
    }
}

/// Looks for WraparoundGhosts whose positions are past the border
/// These entities are despawned
fn stop_wraparound(
    mut commands: Commands,
    q: Query<(Entity, &WraparoundGhost, &Position)>,
    bd: Res<BorderDistance>,
    parent_ent_q: Query<Entity, (With<IsWrapped>, Without<WraparoundGhost>)>,
) {
    for (ghost_ent, wp, pos) in q.iter() {
        let mut despawn = false;
        if wp.offset > 0.0 {
            // This ghost is on the right side of the screen
            if pos.0.x > bd.0 / 2.0 + GHOST_WRAPAROUND_THRESH {
                despawn = true;
            }
        } else {
            // This ghost is on the left side of the screen
            if pos.0.x < -bd.0 / 2.0 - GHOST_WRAPAROUND_THRESH {
                despawn = true;
            }
        }

        if despawn {
            commands.entity(ghost_ent).despawn();
            if let Ok(parent_ent) = parent_ent_q.get(wp.parent) {
                commands.entity(parent_ent).remove::<IsWrapped>();
            }
            //println!("Despawned wraparound ghost");
        }
    }
}

fn wraparound_ghost_position_sync(
    mut q: Query<(&mut Position, &WraparoundGhost)>,
    pos_q: Query<&Position, (Without<WraparoundGhost>)>,
) {
    for (mut ghost_pos, wg) in q.iter_mut() {
        if let Ok(pos) = pos_q.get(wg.parent) {
            ghost_pos.0.x = pos.0.x + wg.offset;
            ghost_pos.0.y = pos.0.y;
        } else {
            println!("Could not get parent of a wraparound ghost! ghost should be despawned");
        }
    }
}

fn wraparound_ghost_texture_sync(
    mut q: Query<(&WraparoundGhost, &mut TextureAtlasSprite)>,
    q_parent: Query<&TextureAtlasSprite, (Without<WraparoundGhost>)>,
) {
    for (wg, mut tas_ghost) in q.iter_mut() {
        if let Ok(tas) = q_parent.get(wg.parent) {
            tas_ghost.flip_x = tas.flip_x;
            tas_ghost.flip_y = tas.flip_y;
            tas_ghost.index = tas.index;
        }
    }
}
