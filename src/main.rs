use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
    velocity: Vec2,
}

#[derive(InputAction)]
#[action_output(Vec2)]
struct Movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(EnhancedInputPlugin)
        .add_input_context::<Player>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_player)
        .add_observer(apply_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player {
            speed: 100.0,
            velocity: Vec2::ZERO,
        },
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
        actions!(
            Player[(
                Action::<Movement>::new(),
                Bindings::spawn(Cardinal::wasd_keys(),)
            )]
        ),
    ));
}

fn apply_movement(movement: On<Fire<Movement>>, mut player_query: Query<&mut Player>) {
    if let Ok(mut player) = player_query.get_mut(movement.context) {
        player.velocity = movement.value * player.speed;
    }
}

fn update_player(player_query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
    for (player, mut transform) in player_query {
        let movement = player.velocity * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}
