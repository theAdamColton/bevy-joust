/*
For an entity to exhibit physics,
insert a PhysicsBundle
*/

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use cgmath::{Point2, Vector2};

pub type V2 = Vector2<f32>;

#[derive(SystemSet, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum PhysicsStages {
    /// Finds x_{i+1} from Position, Velocity, and Acceleration
    /// This is where all Positions are updated
    /// HalfVelocities are also updated
    /// Velocities and Accelerations used in this calculation are those
    ///  updated from the previous iteration
    CalculateNextPositions, 
    /// The next forces are all zeroed
    ClearNextForces,
    /// The next forces are updated, next forces are based on gravity, collision, player movement, ai movement, etc...
    /// Feel free to mutate forces by adding your own forces
    CalculateNextForces,
    /// The next accelerations are calculated from force and mass
    CalculateNextAccelerations,
    /// The next iteration's velocities are calculated from HalfVelocities and the next Accelerations
    CalculateNextVelocities,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        use PhysicsStages::*;
        app.insert_resource(Gravity(V2::new(0.0, -9.81)));
        app.insert_resource(PhysicsScale(15.0));
        app.add_systems(
            Update,
            (
                (
                    (calculate_next_positions).in_set(CalculateNextPositions),
                    (position_sync).before(CalculateNextForces)
                ).chain(),
                (
                    clear_forces.in_set(ClearNextForces),
                    gravity.in_set(CalculateNextForces),
                    calculate_next_accelerations.in_set(CalculateNextAccelerations),
                    calculate_next_velocities.in_set(CalculateNextVelocities),
                ).chain()
            )
        );

        app.configure_sets(Update, (
                CalculateNextPositions,
                ClearNextForces,
                CalculateNextForces,
                CalculateNextAccelerations,
                CalculateNextVelocities
                ).chain()
            );
    }
}

#[derive(Bundle)]
pub struct PhysicsBodyBundle {
    pub m: Mass,
    pub p: Position,
    pub v: Velocity,
    pub hv: HalfVelocity,
    pub a: Acceleration,
    pub f: Force,
}
impl Default for PhysicsBodyBundle {
    fn default() -> Self {
        Self {
            m: Mass(1.0),
            p: Position(V2::new(0.0,0.0)),
            v: Velocity(V2::new(0.0,0.0)),
            hv: HalfVelocity(V2::new(0.0, 0.0)),
            a: Acceleration(V2::new(0.0, 0.0)),
            f: Force(V2::new(0.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub struct StaticPhysicsBodyBundle {
    pub p: Position,
    pub v: Velocity,
    pub hv: HalfVelocity,
    pub a: Acceleration,
    pub _s: StaticObject,
}
impl Default for StaticPhysicsBodyBundle {
    fn default() -> Self {
        Self {
            p: Position(V2::new(0.0,0.0)),
            v: Velocity(V2::new(0.0,0.0)),
            hv: HalfVelocity(V2::new(0.0,0.0)),
            a: Acceleration(V2::new(0.0,0.0)),
            _s: StaticObject,
        }
    }
}

/// All bevy transforms are multiplied by this when converting to Positions
#[derive(Resource)]
pub struct PhysicsScale(pub f32);
/// Indicates that this entity cannot be acted upon
#[derive(Component)]
pub struct StaticObject;
#[derive(Deref, DerefMut, Resource)]
pub struct Gravity(pub V2);
#[derive(Component, Deref, DerefMut)]
pub struct Mass(pub f32);
#[derive(Component, Deref, DerefMut)]
pub struct Position(pub V2);
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub V2);
/// The velocity of halfway between this iteration and the next iteration
#[derive(Component, Deref, DerefMut)]
pub struct HalfVelocity(pub V2);
#[derive(Component, Copy, Clone, Deref, DerefMut)]
pub struct Acceleration(pub V2);
#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Force(pub V2);

fn calculate_next_positions(mut q: Query<(&mut Position, &mut HalfVelocity, &Velocity, &Acceleration)>, t: Res<Time>){
    let dt = t.delta_seconds();
    for (mut pos, mut ha, v, a) in q.iter_mut() {
        (pos.0, ha.0) = next_position_and_half_velocity(pos.0, v.0, a.0, dt);
    }
}

fn clear_forces(mut q: Query<&mut Force>) {
    for mut f in q.iter_mut() {
        f.0 = V2::new(0.0,0.0);
    }
}

/// Goes under calculate next forces
fn gravity(mut q: Query<(&mut Force, &Mass), Without<StaticObject>>, g: Res<Gravity>) {
    for (mut f, m) in q.iter_mut() {
        f.0 += m.0 * g.0;
    }
}

fn calculate_next_accelerations(mut q: Query<(&mut Acceleration, &Force, &Mass), Without<StaticObject>>, t: Res<Time>) {
    let dt = t.delta_seconds();
    for (mut a,f,m) in q.iter_mut() {
        a.0 = f.0 / m.0;
    }
}

fn calculate_next_velocities(mut q: Query<(&mut Velocity, &HalfVelocity, &Acceleration)>, t: Res<Time>)  {
    let dt = t.delta_seconds();
    for (mut v, vh, a) in q.iter_mut() {
        v.0 = next_velocity(vh.0, a.0, dt);
    }
}

fn position_sync(mut q: Query<(&mut Transform, &Position), With<TextureAtlasSprite>>, scale: Res<PhysicsScale>) {
    for (mut t, p) in q.iter_mut() {
        t.translation.x = p.0.x * scale.0;
        t.translation.y = p.0.y * scale.0;
    }
}

/// Leapfrog integration
/// v_{i+1/2} = v_i + a_i * dt / 2
/// x_{i+1} = x_i + v_{i+1/2} * dt
fn next_position_and_half_velocity(x_i: V2, v_i: V2, a_i: V2, dt: f32) -> (V2, V2) {
    let v_i_plus_half = v_i + a_i * dt / 2.0;
    let x_i_plus_1 = x_i + v_i_plus_half * dt;
    (x_i_plus_1, v_i_plus_half)
}

/// Leapfrog integration
/// v_{i+1} = v_{i+1/2} + a_{i+1} * dt / 2
fn next_velocity(v_i_plus_half: V2, a_i_plus_one: V2, dt: f32) -> V2 {
    let v_i_plus_one = v_i_plus_half + a_i_plus_one * dt / 2.0;
    v_i_plus_one
}
