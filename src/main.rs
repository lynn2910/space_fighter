use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};
use crate::background::BackgroundPlugin;
use crate::constants::CAMERA_DEPTH;
use crate::projectiles::ProjectilePlugin;
use crate::shuttle::ShuttlePlugin;

mod constants;
mod shuttle;
mod background;
mod projectiles;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            present_mode: PresentMode::AutoVsync,
                            resolution: WindowResolution::new(constants::LOGICAL_WIDTH, constants::LOGICAL_HEIGHT),
                            title: constants::WINDOW_TITLE.into(),
                            resizable: false,
                            mode: WindowMode::Windowed,
                            position: {
                                let mut p = WindowPosition::default();
                                p.center(MonitorSelection::Primary);
                                p
                            },
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
        )
        .add_plugins((ShuttlePlugin, BackgroundPlugin, ProjectilePlugin))
        .add_systems(Startup, setup_scene)
        .run()
}

fn setup_scene(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_translation(CAMERA_DEPTH)
                .looking_at(Vec3::ZERO, constants::VERTICAL_AXIS),
            ..default()
        },
        BloomSettings::default(),
    ));
    
    // commands.spawn(
    //     BloomSettings {
    //         intensity: 0.15,
    //         low_frequency_boost: 0.7,
    //         low_frequency_boost_curvature: 0.95,
    //         high_pass_frequency: 1.,
    //         composite_mode: BloomCompositeMode::Additive,
    //         ..default()
    //     }
    // );
}