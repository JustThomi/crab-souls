use bevy::prelude::*;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.insert_resource(AmbientLight {
            color: Color::WHITE,    
            brightness: 200.0,
        });
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
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(floor_size))),
            material: floor_material,
            ..default()
        },
    );

    command.spawn(floor);
}
