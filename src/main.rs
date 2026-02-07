use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let status_node = (
        Node {
            width: auto(),
            height: percent(50.),
            left: percent(10.),
            margin: UiRect::vertical(auto()),
            display: Display::Block,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.9, 0.5, 0.5, 0.5)),
        children![
            (Text::new("Turn 10"), TextColor::BLACK),
            (Text::new("Score 1000"), TextColor::BLACK),
            (Text::new("Suspicion 100"), TextColor::BLACK),
        ],
    );

    let patient_node = (
        Node {
            width: auto(),
            height: percent(50.),
            right: percent(10.),
            margin: UiRect::vertical(auto()),
            display: Display::Block,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.5, 0.9, 0.5, 0.5)),
        children![
            (Text::new("Name: Jane Doe"), TextColor::BLACK),
            (Text::new("Health: 100%"), TextColor::BLACK),
            (Text::new("Delirium: 1%"), TextColor::BLACK),
        ],
    );

    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
        children![
            (status_node),
            (patient_node),
            (
                // Next turn
                Node {
                    width: px(100),
                    height: px(100),
                    right: percent(10.),
                    bottom: percent(10.),
                    display: Display::Block,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.5, 0.9, 0.5, 0.5)),
                children![(Text::new("End Turn"), TextColor::BLACK)],
            )
        ],
    ));
}
