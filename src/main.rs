use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(1.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("scene.gltf#Scene0"),
        ..default()
    });
}
