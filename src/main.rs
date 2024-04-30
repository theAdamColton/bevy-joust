#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

mod engine;
mod animation;
mod constants;
mod entities;
mod behavior;
mod player;

use crate::engine::DefaultEnginePlugins;
use crate::engine::physics::V2;
use crate::player::PlayerPluginGroup;
use crate::behavior::BehaviorPlugins;
use animation::AnimationPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use entities::{spritesheets::SpriteSheetPluginState, rider::{BlueRiderSpawnEvent, YellowRiderSpawnEvent, RiderSpawnEventDetails}, platform::{MediumPlatformSpawnEvent, PlatformSpawnEventDetails, BottomPlatformSpawnEvent}, JoustEntitiesPlugins, pter::PterSpawnEvent};
use player::{player_control::PlayerControllerBundle, PlayerBundle};

fn main() {
    App::new()
//        .add_plugins(WindowPlugin {
//            primary_window: Some(Window{
//                resolution: (700., 700.).into(),
//                resizable: false,
//                ..default()
//            }),
//            ..default()
//        })
        .add_plugins(DefaultPlugins)

        // Entities
        .add_plugins(JoustEntitiesPlugins)
        // Behavior
        .add_plugins(BehaviorPlugins)
        // Animation
        .add_plugins(AnimationPlugin)
        // Engine
        .add_plugins(DefaultEnginePlugins)
        // Player control and other player specific plugins
        .add_plugins(PlayerPluginGroup)
        // Debug
        .add_plugins(WorldInspectorPlugin::new())
        // Setup test scene
        .add_systems(
            Startup,
                (test_spawn_riders,
                spawn_bottom_platform,
                spawn_medium_platform,
                setup_camera,
                spawn_pter,
                spawn_eggman)
        )
        .run();
}

fn test_spawn_riders(
    mut event_b: EventWriter<BlueRiderSpawnEvent>,
    mut event_y: EventWriter<YellowRiderSpawnEvent>,
) {
    for x in 0..1 {
        event_b.send(BlueRiderSpawnEvent(RiderSpawnEventDetails {
            position: V2::new(x as f32 * 3.0, 10.0),
            velocity: V2::new(0.0, 0.0),
            optional_player: Some(PlayerBundle::default()),
        }));
    }
    //    event_y.send(YellowRiderSpawnEvent(RiderSpawnEventDetails {
    //        position: V2::new(-5.0, 5.0),
    //        velocity: V2::new(0.0, 0.0),
    //        ..Default::default()
    //    }));
}

fn spawn_medium_platform(mut event_w: EventWriter<MediumPlatformSpawnEvent>) {
    event_w.send(MediumPlatformSpawnEvent(PlatformSpawnEventDetails {
        position: V2::new(15.0,15.0),
        velocity: V2::new(0.0,0.0)
    }))
}

fn spawn_bottom_platform(mut event_w: EventWriter<BottomPlatformSpawnEvent>) {
    event_w.send(BottomPlatformSpawnEvent(PlatformSpawnEventDetails {
        position: V2::new(0.0, -22.0),
        velocity: V2::new(0.0, 0.0),
    }))
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection { near: -1000., far: 1000., ..default()},
        ..default()
    });
}

fn spawn_pter(mut event_w: EventWriter<PterSpawnEvent>) {
    event_w.send(PterSpawnEvent(entities::pter::PterSpawnEventDetails{
        pos: V2::new(0.0,0.0),
        vel: V2::new(10.0,0.0),
    }));
}

fn spawn_eggman(mut event_w: EventWriter<entities::eggman::EggmanSpawnEvent>) {
    event_w.send(entities::eggman::EggmanSpawnEvent {
        position: V2::new(8.0,0.0),
        velocity: V2::new(0.0,0.0),
    });
}
