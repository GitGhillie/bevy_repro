//! A simple 3D scene with light shining over a cube sitting on a plane.

mod camera_control;

use bevy::math::cubic_splines::LinearSpline;
use bevy::math::vec2;
use bevy::{
    core_pipeline::auto_exposure::{
        AutoExposureCompensationCurve, AutoExposurePlugin, AutoExposureSettings,
    },
    prelude::*,
};
use std::f32::consts::PI;

use camera_control::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraControllerPlugin, AutoExposurePlugin))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    asset_server: Res<AssetServer>,
) {
    // gltf
    commands.spawn(SceneBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        scene: asset_server.load("scene.gltf#Scene0"),
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        ..default()
    });

    let walk_speed = 15.0;
    let camera_controller = CameraController {
        walk_speed,
        run_speed: 3.0 * walk_speed,
        ..default()
    };

    // Display the controls of the scene viewer
    info!("{}", camera_controller);

    // auto exposure
    let metering_mask = asset_server.load("textures/basic_metering_mask.png");

    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(15.0, 1.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        AutoExposureSettings {
            compensation_curve: compensation_curves.add(
                AutoExposureCompensationCurve::from_curve(LinearSpline::new([
                    vec2(-4.0, -2.0),
                    vec2(0.0, 0.0),
                    vec2(2.0, 0.0),
                    vec2(4.0, 2.0),
                ]))
                .unwrap(),
            ),
            speed_brighten: 1.5,
            speed_darken: 0.5,
            metering_mask: metering_mask.clone(),
            ..default()
        },
        camera_controller,
    ));
}
