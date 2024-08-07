use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn, spawn_light));
    }
}

fn spawn(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_size: Vec3 = Vec3 {
        x: 20.0,
        y: 1.0,
        z: 20.0,
    };

    let floor_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(20.0, 200.0, 20.0),
        ..default()
    });

    let floor = (
        RigidBody::Fixed,
        Collider::cuboid(floor_size.x / 2.0, floor_size.y / 2.0, floor_size.z / 2.0),
        Friction::coefficient(2.5),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(floor_size))),
            material: floor_material,
            ..default()
        },
    );

    command.spawn(floor);
}

fn spawn_light(mut command: Commands) {
    let light: PointLightBundle = PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        point_light: PointLight {
            intensity: 10_000.0,
            color: WHITE.into(),
            shadows_enabled: true,
            ..default()
        },
        ..default()
    };

    command.spawn(light);
}
