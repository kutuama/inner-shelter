use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Inner Shelter".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .run();
}

/// Sets up the initial game state, including camera, lighting, and two box entities.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn a point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn the first box (Red)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(1.0) })), // Use Cuboid with half_size
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2), // Updated to srgb
            ..default()
        }),
        transform: Transform::from_xyz(-3.0, 1.0, 0.0),
        ..default()
    });

    // Spawn the second box (Green)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(1.0) })), // Use Cuboid with half_size
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2), // Updated to srgb
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..default()
    });
}
