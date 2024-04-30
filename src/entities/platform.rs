use crate::engine::collision::{ColliderBundle, SquareCollider};
use crate::constants::*;
use crate::engine::physics::{Position, StaticPhysicsBodyBundle, Velocity, V2};
use crate::entities::spritesheets::*;
use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MediumPlatformSpawnEvent>();
        app.add_event::<BottomPlatformSpawnEvent>();
        app.add_systems(Update, (medium_platform_listener, bottom_platform_listener));
    }
}

#[derive(Bundle)]
pub struct PlatformBundle {
    pub sprite: SpriteSheetBundle,
    _platform: PlatformSprite,
    cb: ColliderBundle,
    spbb: StaticPhysicsBodyBundle,
}

#[derive(Component)]
pub struct PlatformSprite;

pub struct PlatformSpawnEventDetails {
    pub position: V2,
    pub velocity: V2,
}

#[derive(Event)]
pub struct MediumPlatformSpawnEvent(pub PlatformSpawnEventDetails);
#[derive(Event)]
pub struct BottomPlatformSpawnEvent(pub PlatformSpawnEventDetails);

fn medium_platform_listener(
    mut commands: Commands,
    mut events: EventReader<MediumPlatformSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<platform_kinds::PlatformMedium>>,
) {
    for e in events.iter() {
        let tex = q.single();
        let scale = 1.5;
        let coll = SquareCollider {
            min: V2::new(-7.5, -2.2),
            max: V2::new(15.0, 1.8),
            offset: V2::new(0.0,0.0),
            ..Default::default()
        };
        spawn_platform(&mut commands, tex, e.0.position, e.0.velocity, coll, scale);
    }
}
fn bottom_platform_listener(
    mut commands: Commands,
    mut events: EventReader<BottomPlatformSpawnEvent>,
    q: Query<&Handle<TextureAtlas>, With<platform_kinds::PlatformBottom>>,
) {
    for e in events.iter() {
        let tex = q.single();
        let scale = 1.5;
        let coll = SquareCollider {
            bounce: 1.0,
            min: V2::new(-23.9, -3.0),
            max: V2::new(23.9, 0.0),
            offset: V2::new(0.0, 4.0),
            ..Default::default()
        };
        spawn_platform(&mut commands, tex, e.0.position, e.0.velocity, coll, scale);
    }
}

fn spawn_platform<'a, 'b>(
    commands: &mut Commands<'b, 'a>,
    texture_at: &Handle<TextureAtlas>,
    pos: V2,
    vel: V2,
    coll: SquareCollider,
    scale: f32,
) -> Entity {
    let ssb = SpriteSheetBundle {
        texture_atlas: texture_at.clone(),
        transform: Transform {
            // TODO tweak translation
            //translation: ,
            // TODO tweak scale
            scale: Vec3::splat(scale * GLOBAL_SPRITE_SCALE),
            ..Default::default()
        },
        ..Default::default()
    };

    let cb = ColliderBundle {
        sq: coll,
        ..Default::default()
    };

    let spbb = StaticPhysicsBodyBundle {
        p: Position(pos),
        v: Velocity(vel),
        ..Default::default()
    };

    let pb = PlatformBundle {
        _platform: PlatformSprite,
        sprite: ssb,
        cb,
        spbb,
    };

    commands.spawn(pb).id()
}
