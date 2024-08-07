use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut command: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            cursor_lock_key: KeyCode::Escape,
            zoom: Zoom::new(8.0, 8.0),
            ..default()
        },
    );

    command.spawn(camera);
}
