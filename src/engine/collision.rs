use crate::engine::physics::{
    Acceleration, Force, PhysicsStages, Position, StaticObject, Velocity, V2,
};
use crate::engine::wraparound::coord_space_to_wraparound_space;
use crate::engine::wraparound::BorderDistance;
use bevy::prelude::*;
use cgmath::prelude::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        use PhysicsStages::*;
        app.add_event::<StaticCollisionEvent>();
        app.add_systems(Update, 
            ((
                compute_static_collider_forces,
                compute_non_static_intersections
            ).before(CalculateNextPositions).in_set(CalculateCollisions),
            (
                clear_collider_forces.in_set(ClearNextForces),
                apply_collider_functions.in_set(CalculateNextForces),
            ).chain(),
            clear_grounded,
            debug_print_grounded
            )
        );
    }
}

/// Collisions for this frame are calculated. After this
/// stage is done it is safe to use collision results
#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub struct CalculateCollisions;

#[derive(Event)]
pub struct StaticCollisionEvent {
    e1: Entity,
    e2: Entity,
}

#[derive(Component, Clone, Copy)]
pub struct SquareCollider {
    pub min: V2,
    pub max: V2,
    /// When attaching this collider to a Position,
    /// what should the offset be from the position
    pub offset: V2,
    pub bounce: f32,
}
impl SquareCollider {
    /// Returns adjusted (min,max) based on the border and the center point
    /// coordinates will be returned in `wraparound space`
    fn border_adjusted_bounds(&self, border_distance: f32, center_point: V2) -> (V2, V2) {
        let mut min = self.min + center_point + self.offset;
        let mut max = self.max + center_point + self.offset;
        min.x = coord_space_to_wraparound_space(min.x, border_distance);
        max.x = coord_space_to_wraparound_space(max.x, border_distance);
        (min, max)
    }
}
impl Default for SquareCollider {
    fn default() -> Self {
        Self {
            min: V2::new(-1.0, -1.0),
            max: V2::new(1.0, 1.0),
            offset: V2::new(0.0, 0.0),
            bounce: 1.0,
        }
    }
}

#[derive(Component)]
pub struct Collideable;

#[derive(Component)]
pub struct ColliderForces(V2);

#[derive(Bundle)]
pub struct ColliderBundle {
    pub sq: SquareCollider,
    pub _coll: Collideable,
    pub cf: ColliderForces,
}
impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            sq: SquareCollider::default(),
            _coll: Collideable,
            cf: ColliderForces(V2::new(0.0, 0.0)),
        }
    }
}

/// Indicates that this entity will have its square collider 
/// compared with all other (non static) square colliders every frame.
/// No more than on single intersection will ever be updated on a frame.
/// The intersecting entity is inserted as a NonStaticCollision Component.
#[derive(Component, Copy, Clone)]
pub struct ShouldCalculateNonStaticIntersectionsOn;
/// This is inserted into entities with ShouldCalculateNonStaticIntersectionsOn
/// when they intersect with another non static square collider
#[derive(Component)]
pub struct NonStaticCollisionEvent(pub Entity);

fn compute_non_static_intersections(
    mut commands: Commands,
    q1: Query<(Entity, &SquareCollider, &Position), (Without<StaticObject>, With<ShouldCalculateNonStaticIntersectionsOn>)>,
    q2: Query<(Entity, &SquareCollider, &Position), Without<StaticObject>, >,
    //mut event_non_static_collision: EventWriter<NonStaticCollisionEvent>,
    border_distance: Res<BorderDistance>,
) {
    for (ent1, sc1, pos1) in q1.iter() {
        let adjusted_bounds_1 = sc1.border_adjusted_bounds(border_distance.0, pos1.0);
        for (ent2, sc2, pos2) in q2.iter() {
            if ent1.index() == ent2.index() {
                continue;
            }
            let adjusted_bounds_2 = sc2.border_adjusted_bounds(border_distance.0, pos2.0);

            let sv = compute_separation_vector(
                adjusted_bounds_1.0,
                adjusted_bounds_1.1,
                adjusted_bounds_2.0,
                adjusted_bounds_2.1,
                border_distance.0,
            );

            if sv.magnitude() > 0.0 {
                // Collision occurred
                //event_non_static_collision.send(NonStaticCollisionEvent{ent1, ent2});
                commands.entity(ent1).insert(NonStaticCollisionEvent(ent2));
            }
        }
    }
}

/// Computes the ColliderForces for all SquareColliders colliding with static objects
/// https://2dengine.com/?p=collisions
fn compute_static_collider_forces(
    mut q1: Query<
        (
            Entity,
            &SquareCollider,
            &mut Position,
            &mut Velocity,
            Option<&mut Grounded>,
        ),
        (With<Collideable>, Without<StaticObject>),
    >,
    q2: Query<
        (Entity, &SquareCollider, &Position, &Velocity),
        (With<Collideable>, With<StaticObject>),
    >,
    mut er: EventWriter<StaticCollisionEvent>,
    border_distance: Res<BorderDistance>,
) {
    for (ent1, sc1, mut pos1, mut vel1, mut maybe_grounded) in q1.iter_mut() {
        let adjusted_bounds_1 = sc1.border_adjusted_bounds(border_distance.0, pos1.0);
        for (ent2, sc2, pos2, vel2) in q2.iter() {
            let adjusted_bounds_2 = sc2.border_adjusted_bounds(border_distance.0, pos2.0);

            let mut sv = compute_separation_vector(
                adjusted_bounds_1.0,
                adjusted_bounds_1.1,
                adjusted_bounds_2.0,
                adjusted_bounds_2.1,
                border_distance.0,
            );

            if sv.magnitude() > 0.0 {
                er.send(StaticCollisionEvent { e1: ent1, e2: ent2 });
                // union should point from 1 to 2
                if (pos1.0 - pos2.0).x < 0.0 {
                    sv.x *= -1.0;
                }
                if (pos1.0 - pos2.0).y < 0.0 {
                    sv.y *= -1.0
                }
                // Relative velocity
                let rel_v = vel1.0 - vel2.0;

                // Will only move in one direction: whichever is the shorter distance to be
                // moved, and if the two colliders are moving towards each other
                if sv.x.abs() < sv.y.abs() {
                    if sv.x * rel_v.x < 0.0 {
                        pos1.0.x += sv.x;
                        //println!("Fixing pos.x by {}", sv.x);
                        vel1.0.x -= rel_v.x;
                        //println!("Fixing vel.x by {}", rel_v.x);
                    }
                } else {
                    if sv.y * rel_v.y < 0.0 {
                        // Moving towards each other
                        pos1.0.y += sv.y;
                        //println!("Fixing pos.y by {}", sv.y);
                        vel1.0.y -= rel_v.y;
                        //println!("Fixing vel.y by {}", rel_v.y);
                    }
                    if sv.y > 0.0 {
                        // Sets grounded
                        let grounded = maybe_grounded.take();
                        if let Some(mut g) = grounded {
                            g.0 = GroundedState::GroundedTo(ent2);
                        }
                    }
                }

                //                println!(
                //                    "sv: {:?}, sc1: {:?} {:?} sc2: {:?} {:?} ps {:.2} rel_v {:?}",
                //                    sv, sc1.min, sc1.max, sc2.min, sc2.max, ps, rel_v
                //                );
            }
        }
    }
}

/// What is the shortest x component and y component that would separate the two
/// rectangles?
///
/// This calculation assumes that the points are in `wraparound space`
/// `wraparound space` is a coordinate system where the max value is no greater than the right border,
/// and the min value is no less than the left border. Border x positions are determined by 1/2 `border_distance`
fn compute_separation_vector(min1: V2, max1: V2, min2: V2, max2: V2, border_distance: f32) -> V2 {
    let sx = axis_collision_wraparound(min1.x, max1.x, min2.x, max2.x, border_distance);
    let sy = axis_collision(min1.y, max1.y, min2.y, max2.y);

    if sx == 0.0 || sy == 0.0 {
        return V2::zero();
    }
    V2::new(sx, sy)
}

/// Which direction should l1 and h1 be moved such that they no longer collide with l2 and h2
fn axis_collision(l1: f32, h1: f32, l2: f32, h2: f32) -> f32 {
    // Four collision cases...
    if l1 < l2 && l2 < h1 && h1 < h2 {
        // 1 is to the left of 2 and not enclosed
        return l2 - h1;
    } else if l2 < l1 && l1 < h1 && h1 < h2 {
        // 1 is enclosed by 2
        let r = h2 - l1;
        let l = l2 - h1;
        return f32::min(r.abs(), l.abs());
    } else if l1 < l2 && l2 < h2 && h2 < h1 {
        // 2 is enclosed by 1
        let r = h2 - l1;
        let l = l2 - h1;
        return f32::min(r.abs(), l.abs());
    } else if l2 < l1 && l1 < h2 && h2 < h1 {
        // 1 is to the right of 2 and not enclosed
        return h2 - l1;
    } else {
        return 0.0;
    }
}

/// Which direction should l1 and h1 be moved such that they no longer collide with l2 and h2
/// This function depends on a wraparound coordinate system
///     Wraparound separation vectors can technically be infinite length,
///     because of the inherent ring shape. This function will check for that
///     case and return infinity.
/// This function expects that the points are from 0.0 to world_width
fn axis_collision_wraparound(
    mut l1: f32,
    mut h1: f32,
    mut l2: f32,
    mut h2: f32,
    world_width: f32,
) -> f32 {
    // First, the points are set up to be in the correct order of magnitude

    //println!("axis_coll_x {l1} {h1} {l2} {h2}");

    // Is 1 wrapped around the world border?
    if l1 > h1 {
        //println!("1 is wrapped");
        // Scales all points apart from h1
        l1 -= world_width;
        l2 -= world_width;
        h2 -= world_width;
    }
    // Is 2 wrapped around the world border?
    if l2 > h2 {
        //println!("2 is wrapped: l1 {l1} h1 {h1} l2 {l2} h2 {h2}");
        // Scales all points apart from h2
        l2 -= world_width;
        l1 -= world_width;
        h1 -= world_width;
    }

    if (h1 - l1) + (h2 - l2) > world_width {
        //println!("Infinite separation on x axis!");
        return f32::INFINITY;
    }

    axis_collision(l1, h1, l2, h2)
}

/// Updates the accelerations of all colliders based on their stored collider forces
fn apply_collider_functions(
    mut q: Query<(&mut Force, &ColliderForces, &SquareCollider), With<Collideable>>,
    t: Res<Time>,
) {
    let dt = t.delta_seconds();
    for (mut force, cf, sc) in q.iter_mut() {
        force.0 += cf.0 * sc.bounce;
    }
}

fn clear_collider_forces(mut q: Query<&mut ColliderForces>) {
    for mut cf in q.iter_mut() {
        cf.0 = V2::new(0.0, 0.0);
    }
}

/// For entities that want to know if they are gounded
#[derive(Component)]
pub struct Grounded(pub GroundedState);
#[derive(Component)]
pub enum GroundedState {
    NotGrounded,
    /// The entity is the static object this is grounded to
    GroundedTo(Entity),
}

/// Checks to see if any grounded entities are not next to their grounded static objects anymore
fn clear_grounded(
    mut q: Query<(&mut Grounded, &Position, &SquareCollider), Without<StaticObject>>,
    q_static: Query<(&Position, &SquareCollider), With<StaticObject>>,
    border_distance: Res<BorderDistance>,
) {
    use GroundedState::*;
    for (mut g, pos1, sq1) in q.iter_mut() {
        let adjusted_bounds_1 = sq1.border_adjusted_bounds(border_distance.0, pos1.0);
        if let GroundedTo(gs) = g.0 {
            // Check if the static object in gs is colliding with this
            if let Ok((pos2, sq2)) = q_static.get(gs) {
                let adjusted_bounds_2 = sq2.border_adjusted_bounds(border_distance.0, pos2.0);
                // How much wiggle room before g is not grounded
                let padding = V2::new(0.1, 0.1);

                let sv_y = axis_collision(
                    adjusted_bounds_1.0.y,
                    adjusted_bounds_1.1.y,
                    adjusted_bounds_2.0.y,
                    adjusted_bounds_2.1.y,
                );

                if sv_y == 0.0 {
                    g.0 = GroundedState::NotGrounded;
                }
            }
        }
    }
}

fn debug_print_grounded(q: Query<(Entity, &Grounded)>) {
    for (e, g) in q.iter() {
        match &g.0 {
            GroundedState::GroundedTo(e2) => {
                println!("{:?} is grounded to {:?}", e, e2);
            }
            GroundedState::NotGrounded => {
                println!("{:?} is not grounded", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_axis_collision_wraparound() {
    /*
    This test demonstrates how collision with wraparound works. First the square colliders are made,
    next, the square collider min max points are put into the wraparound space
    then, the axis_collision_wraparound is computed to get the separation vector for the x values
     */
    let sq1 = SquareCollider {
        min: V2::new(-1.0, 0.0),
        max: V2::new(1.0, 0.0),
        bounce: 0.0,
        offset: V2::zero(),
    };
    let sq2 = SquareCollider {
        min: V2::new(-1.0, 0.0),
        max: V2::new(2.0, 0.0),
        bounce: 0.0,
        offset: V2::zero(),
    };
    let res1 = sq1.border_adjusted_bounds(24.0, V2::new(-12.0, 0.0));
    let res2 = sq1.border_adjusted_bounds(24.0, V2::new(-11.0, 0.0));
    let ax_c = axis_collision_wraparound(res1.0.x, res1.1.x, res2.0.x, res2.1.x, 24.0);
    println!("res1: {:?} res2: {:?}", res1, res2);
    println!("ax_c x: {:?}", ax_c);
}
