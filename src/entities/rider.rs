use crate::engine::collision::*;
use crate::constants::*;
use crate::engine::physics::*;
use crate::animation::rider_animation::RiderAnimationBundle;
use crate::entities::spritesheets::*;
use crate::behavior::movement_control::MovementControl;
use crate::player::PlayerBundle;
use crate::player::player_control::PlayerControllerBundle;

use bevy::prelude::*;

pub struct RiderPlugin;
impl Plugin for RiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RedRiderSpawnEvent>();
        app.add_event::<GreyRiderSpawnEvent>();
        app.add_event::<YellowRiderSpawnEvent>();
        app.add_event::<IndigoRiderSpawnEvent>();
        app.add_event::<BlueRiderSpawnEvent>();
        app.add_systems(Update,
            (red_rider_listener,
             grey_rider_listener,
             yellow_rider_listener,
             indigo_rider_listener,
             blue_rider_listener
             ));
    }
}


#[derive(Bundle)]
pub struct RiderBundle {
    pub sprite: SpriteSheetBundle,
    rider: RiderSprite,
    pub coll_bundle: ColliderBundle,
    pub phys_b: PhysicsBodyBundle,
    pub grounded: Grounded,
    pub rab: RiderAnimationBundle,
    mc: MovementControl,
}

#[derive(Component)]
pub struct RiderSprite;

/// Indicates that this rider has been toppled, and will render as a mount
#[derive(Component)]
pub struct IsRespawning;

/// Information needed to spawn a new rider of any type
pub struct RiderSpawnEventDetails {
    pub position: V2,
    pub velocity: V2,
    pub optional_player: Option<PlayerBundle>,
}
impl Default for RiderSpawnEventDetails {
    fn default() -> Self {
        Self {
            position: V2::new(0.0, 0.0),
            velocity: V2::new(0.0,0.0),
            optional_player: None,
        }
    }
}

// Spawn events for riders

#[derive(Event)]
pub struct RedRiderSpawnEvent(pub RiderSpawnEventDetails);
#[derive(Event)]
pub struct GreyRiderSpawnEvent(pub RiderSpawnEventDetails);
#[derive(Event)]
pub struct YellowRiderSpawnEvent(pub RiderSpawnEventDetails);
#[derive(Event)]
pub struct IndigoRiderSpawnEvent(pub RiderSpawnEventDetails);
#[derive(Event)]
pub struct BlueRiderSpawnEvent(pub RiderSpawnEventDetails);

// Spawner event listeners

fn red_rider_listener(
    mut commands: Commands,
    mut events: EventReader<RedRiderSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<rider_kinds::RedRider>>,
) {
    for e in events.iter() {
        let tex = q.single();
        spawn_rider(&mut commands, tex, &e.0);
    }
}
fn grey_rider_listener(
    mut commands: Commands,
    mut events: EventReader<GreyRiderSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<rider_kinds::GreyRider>>,
) {
    for e in events.iter() {
        let tex = q.single();
        spawn_rider(&mut commands, tex, &e.0);
    }
}
fn yellow_rider_listener(
    mut commands: Commands,
    mut events: EventReader<YellowRiderSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<rider_kinds::YellowRider>>,
) {
    for e in events.iter() {
        let tex = q.single();
        spawn_rider(&mut commands, tex, &e.0);
    }
}
fn indigo_rider_listener(
    mut commands: Commands,
    mut events: EventReader<IndigoRiderSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<rider_kinds::IndigoRider>>,
) {
    for e in events.iter() {
        let tex = q.single();
        spawn_rider(&mut commands, tex, &e.0);
    }
}
fn blue_rider_listener(
    mut commands: Commands,
    mut events: EventReader<BlueRiderSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<rider_kinds::BlueRider>>,
) {
    for e in events.iter() {
        let tex = q.single();
        spawn_rider(&mut commands, tex, &e.0);
    }
}

fn spawn_rider<'a, 'b>(
    commands: &mut Commands<'b, 'a>,
    texat_h: &Handle<TextureAtlas>,
    spawn_event_details: &RiderSpawnEventDetails,
) -> Entity {
    let ssb = SpriteSheetBundle {
        texture_atlas: texat_h.clone(),
        transform: Transform {
            //    translation: Vec3::new(pos.x, pos.y, 0.0),
            // TODO teak scale
            scale: Vec3::splat(2.5 * GLOBAL_SPRITE_SCALE),
            ..Default::default()
        },
        ..Default::default()
    };

    let coll = ColliderBundle {
        // TODO figure out bounds
        sq: SquareCollider {
            min: V2::new(-0.5, -0.5),
            max: V2::new(0.5, 0.5),
            offset: V2::new(0.0, 0.0),
            bounce: 1.0,
        },
        ..Default::default()
    };

    let pb = PhysicsBodyBundle {
        m: Mass(10.0),
        p: Position(spawn_event_details.position),
        v: Velocity(spawn_event_details.velocity),
        hv: HalfVelocity(V2::new(0.0, 0.0)),
        a: Acceleration(V2::new(0.0, 0.0)),
        f: Force(V2::new(0.0, 0.0)),
    };

    let mc = MovementControl::default();

    let rb = RiderBundle {
        sprite: ssb,
        rider: RiderSprite,
        coll_bundle: coll,
        phys_b: pb,
        grounded: Grounded(GroundedState::NotGrounded),
        rab: RiderAnimationBundle::default(),
        mc,
    };

    let id = commands.spawn(rb).id();

    if let Some(pcb) = spawn_event_details.optional_player {
        commands.entity(id).insert(pcb);
    }

    id
}
