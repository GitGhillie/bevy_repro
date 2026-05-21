use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(1.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(DirectionalLight {
        shadow_maps_enabled: true,
        ..default()
    });

    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("scene.gltf")),
    ));
}
