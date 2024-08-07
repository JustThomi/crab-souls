use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (movement, jump));
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    jump_force: f32,
}

fn spawn(mut command: Commands, asset_server: Res<AssetServer>) {
    let player_model = asset_server.load("models/player.glb#Scene0");

    let player = (
        Player {
            speed: 10.0,
            jump_force: 0.5,
        },
        ThirdPersonCameraTarget,
        RigidBody::Dynamic,
        GravityScale::default(),
        Collider::capsule_y(0.5, 0.5),
        LockedAxes::ROTATION_LOCKED, // for now
        SceneBundle {
            scene: player_model,
            transform: Transform::from_xyz(1.0, 1.0, 0.0),
            ..default()
        },
        KinematicCharacterController { ..default() },
    );

    command.spawn(player);
}

// this needs a redo lol
fn jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    // time: Res<Time>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            transform.translation.y += player.jump_force;
        }
    }
}

fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut player_query: Query<(&mut KinematicCharacterController, &Player)>,
) {
    let mut direction: Vec3 = Vec3::ZERO;

    if let Ok((mut controller, player)) = player_query.get_single_mut() {
        if let Ok(camera) = camera_query.get_single() {
            if keys.pressed(KeyCode::KeyW) {
                direction += camera.forward().as_vec3();
            }
            if keys.pressed(KeyCode::KeyS) {
                direction += camera.back().as_vec3();
            }
            if keys.pressed(KeyCode::KeyA) {
                direction += camera.left().as_vec3();
            }
            if keys.pressed(KeyCode::KeyD) {
                direction += camera.right().as_vec3();
            }
        }
        direction.y = 0.0; // to stop clipping for now

        controller.translation =
            Some(direction.normalize_or_zero() * player.speed * time.delta_seconds());

        // if direction.length_squared() > 0.0 {
        //     controller.look_at(direction, Vec3::Y); // not for this solution but will keep it for later maybe
        // }
    }
}
