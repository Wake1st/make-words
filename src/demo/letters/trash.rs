use bevy::prelude::*;

use crate::{screens::Screen, AppSet};

use super::window_size::{WindowSize, WindowSizeUpdated};

const TRASH_OFFSET: Vec2 = Vec2::new(32.0, 32.0);
const TRASH_SIZE: Vec2 = Vec2::splat(128.0);
const TRASH_LAYER: f32 = -1.0;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TrashCanDimentions { rect: Rect::EMPTY });

    app.add_systems(OnEnter(Screen::Gameplay), spawn_trash_can)
        .add_systems(Update, move_trash_can.in_set(AppSet::Update));
}

#[derive(Component)]
pub struct TrashCan {}

#[derive(Resource)]
pub struct TrashCanDimentions {
    pub rect: Rect,
}

fn spawn_trash_can(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
    mut trash_can: ResMut<TrashCanDimentions>,
) {
    //  set trash can dimentions
    let trash_can_location = window_size.size / 2.0 - TRASH_OFFSET - TRASH_SIZE / 2.0;
    trash_can.rect = Rect::from_center_size(trash_can_location, TRASH_SIZE);

    let texture = asset_server.load("images/trash.png");

    commands.spawn((
        Name::new("TrashCan"),
        SpriteBundle {
            texture,
            transform: Transform::from_translation(trash_can_location.extend(TRASH_LAYER)),
            ..default()
        },
        TrashCan {},
        StateScoped(Screen::Gameplay),
    ));
}

fn move_trash_can(
    mut window_resize: EventReader<WindowSizeUpdated>,
    mut trash_can_dimentions: ResMut<TrashCanDimentions>,
    mut trash_can: Query<&mut Transform, With<TrashCan>>,
) {
    for event in window_resize.read() {
        let trash_can_location = event.size / 2.0 - TRASH_OFFSET - TRASH_SIZE / 2.0;
        trash_can_dimentions.rect = Rect::from_center_size(trash_can_location, TRASH_SIZE);

        for mut transform in trash_can.iter_mut() {
            transform.translation = trash_can_location.extend(TRASH_LAYER);
        }
    }
}
