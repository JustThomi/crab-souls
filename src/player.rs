use bevy::{
    animation::animate_targets, prelude::*
};
use bevy_third_person_camera::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (movement, handle_animations));
        app.add_systems(Update, setup_animations.before(animate_targets));
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    direction: Vec3,
}

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

fn setup_animations(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[5], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

// primitive animation handling..should probably look for a better solution
fn handle_animations(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut player_query: Query<&Player>,
) {
    for (mut animation_player, mut transitions) in &mut animation_players {
        if let Ok(player) = player_query.get_single_mut() {
            if player.direction != Vec3::ZERO && !animation_player.is_playing_animation(animations.animations[1]){
                transitions
                    .play(
                        &mut animation_player,
                        animations.animations[1], // run
                        Duration::from_millis(250),
                    )
                    .repeat();
            }
            else if player.direction == Vec3::ZERO && !animation_player.is_playing_animation(animations.animations[5]){
                transitions
                    .play(
                        &mut animation_player,
                        animations.animations[5], // idle
                        Duration::from_millis(250),
                    )
                    .repeat();
            }
        }
    }
}

// TODO: look into better collision detection
fn spawn(
    mut command: Commands, 
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let player_model = asset_server.load("models/player.glb#Scene0");
    let mut graph = AnimationGraph::new();

    // TODO: find out why clips "get shuffled"
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(37).from_asset("models/player.glb"), // idle
                GltfAssetLabel::Animation(49).from_asset("models/player.glb"), // run
                GltfAssetLabel::Animation(2).from_asset("models/player.glb"), // attack
                GltfAssetLabel::Animation(21).from_asset("models/player.glb"), // block(shield)
                GltfAssetLabel::Animation(25).from_asset("models/player.glb"), // death
                GltfAssetLabel::Animation(36).from_asset("models/player.glb"), // hit
                GltfAssetLabel::Animation(72).from_asset("models/player.glb"), // heal
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    let graph = graphs.add(graph);

    command.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

    let player = (
        Player {
            speed: 10.0,
            direction: Vec3::ZERO,
        },
        ThirdPersonCameraTarget,
        SceneBundle {
            scene: player_model,
            transform:Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    );

    command.spawn(player);
}

fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let mut direction: Vec3 = Vec3::ZERO;

    if let Ok((mut controller, mut player)) = player_query.get_single_mut() {
        if let Ok(camera) = camera_query.get_single() {
            if keys.pressed(KeyCode::KeyW) {
                direction += camera.forward().normalize();
            }
            if keys.pressed(KeyCode::KeyS) {
                direction -= camera.forward().normalize();
            }
            if keys.pressed(KeyCode::KeyA) {
                direction -= camera.right().normalize();
            }
            if keys.pressed(KeyCode::KeyD) {
                direction += camera.right().normalize();
            }
        }
        direction.y = 0.0; // to stop clipping for now
        player.direction = direction;
        controller.translation += direction * player.speed * time.delta_seconds();
    }
}