use std::collections::VecDeque;

use bevy::asset::AssetMetaCheck;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .init_resource::<Hand>()
        .add_systems(Startup, setup)
        .add_systems(Update, (card_follow_mouse, move_cards))
        .add_observer(on_card_added)
        .add_observer(on_card_removed)
        .run();
}

#[derive(Component)]
pub struct Card;

#[derive(Resource, Default)]
pub struct Hand {
    cards: Vec<Entity>,
}

impl Hand {
    pub fn insert(&mut self, entity: Entity) {
        self.cards.push(entity);
    }

    pub fn remove(&mut self, entity: Entity) {
        self.cards.retain(|el| el.index() != entity.index());
    }

    pub fn offset_of(&self, entity: Entity) -> Option<usize> {
        self.cards
            .iter()
            .position(|el| el.index() == entity.index())
    }
}

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct Patient;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        Pickable::IGNORE,
        children![status_info_node(), patient_info_node(), next_turn_node()],
    ));

    let entity = commands
        .spawn((
            Card,
            Transform::from_xyz(0., 0., 0.),
            Visibility::default(),
            Sprite {
                image: asset_server.load("sprite/slice63.png"),
                ..default()
            },
            Pickable::default(),
            children![(
                Text2d::new("Sedative"),
                Transform::from_xyz(0., 120., 0.1),
                TextColor::BLACK
            )],
        ))
        .observe(hold_card)
        .observe(drop_card)
        .id();

    commands.spawn((
        Patient,
        Transform::default(),
        Visibility::default(),
        children![(
            Sprite {
                image: asset_server.load("sprite/patient.png"),
                ..default()
            },
            Transform::default(),
        ),],
    ));
}

fn on_card_added(add: On<Add, Card>, mut hand: ResMut<Hand>) {
    hand.insert(add.entity);
}

fn on_card_removed(remove: On<Remove, Card>, mut hand: ResMut<Hand>) {
    hand.insert(remove.entity);
}

fn hold_card(
    clicked: On<Pointer<Press>>,
    card_query: Query<Entity, (With<Card>, Without<Held>)>,
    mut commands: Commands,
) {
    if let Ok(entity) = card_query.get(clicked.entity) {
        info!("picked up card");
        commands.get_entity(entity).unwrap().insert(Held);
    }
}

fn drop_card(
    clicked: On<Pointer<Release>>,
    card_query: Query<Entity, (With<Card>, With<Held>)>,
    mut commands: Commands,
) {
    if let Ok(entity) = card_query.get(clicked.entity) {
        info!("dropped card");
        commands.get_entity(entity).unwrap().remove::<Held>();

        // TODO: Check where the card was dropped...
    }
}

/// Moves the held card with the mouse pointer.
fn card_follow_mouse(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut held_query: Query<&mut Transform, With<Held>>,
) {
    if let Ok(mut transform) = held_query.single_mut() {
        let (camera, camera_transform) = *camera_query;
        if let Some(cursor_position) = window.cursor_position()
            && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        {
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
        }
    }
}

/// Makes sure unheld cards are placed in the "hand".
fn move_cards(
    mut card_query: Query<(Entity, &mut Transform), (With<Card>, Without<Held>)>,
    hand: Res<Hand>,
    time: Res<Time>,
) {
    for (entity, mut transform) in card_query {
        let offset = hand.offset_of(entity).unwrap() as f32 * 100.;
        let target = Vec3::new(offset, -500., 0.);
        // transform.translation = target;
        // transform.translation.lerp(target, time.delta_secs());
        transform
            .translation
            .smooth_nudge(&target, 5., time.delta_secs());
    }
}

fn next_turn_node() -> impl Bundle {
    (
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
}

fn status_info_node() -> impl Bundle {
    (
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
    )
}

fn patient_info_node() -> impl Bundle {
    (
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
    )
}
