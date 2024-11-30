use bevy::prelude::*;

use crate::{
    demo::letters::letter_loader::{load_letters, LetterList},
    screens::Screen,
    AppSet,
};

use super::interaction::{CloseDrawer, OpenDrawer};

const SLIDE_DISTANCE: Vec3 = Vec3::new(0.0, 800.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), setup_drawer.after(load_letters))
        .add_systems(
            Update,
            ((show_drawer, hide_drawer), slide_drawer)
                .chain()
                .in_set(AppSet::Update),
        );
}

#[derive(Component)]
pub struct LetterDrawer {
    pub target_position: Vec3,
}

fn setup_drawer(
    mut commands: Commands,
    letter_list: Res<LetterList>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_wrap: FlexWrap::Wrap,
                            row_gap: Val::Px(8.),
                            column_gap: Val::Px(8.),
                            height: Val::Percent(80.),
                            width: Val::Percent(88.),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(-SLIDE_DISTANCE),
                        ..default()
                    },
                    StateScoped(Screen::Gameplay),
                ))
                .with_children(|builder| {
                    for letter in letter_list.letters.iter() {
                        let texture: Handle<Image> =
                            asset_server.load(format!("images/letters/{}", letter.asset_path));

                        builder.spawn((
                            letter.clone(),
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(128.),
                                    height: Val::Px(128.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                transform: Transform::from_scale(Vec2::splat(1.0).extend(0.0)),
                                image: UiImage {
                                    texture,
                                    ..default()
                                },
                                ..default()
                            },
                            StateScoped(Screen::Gameplay),
                        ));
                    }
                });
        });
}

fn show_drawer(
    mut open_drawer: EventReader<OpenDrawer>,
    mut drawer_query: Query<(&mut LetterDrawer, &Transform)>,
) {
    for _ in open_drawer.read() {
        if let Ok((mut drawer, transform)) = drawer_query.get_single_mut() {
            drawer.target_position = transform.translation + SLIDE_DISTANCE;
        }
    }
}

fn hide_drawer(
    mut close_drawer: EventReader<CloseDrawer>,
    mut drawer_query: Query<(&mut LetterDrawer, &Transform)>,
) {
    for _ in close_drawer.read() {
        if let Ok((mut drawer, transform)) = drawer_query.get_single_mut() {
            drawer.target_position = transform.translation - SLIDE_DISTANCE;
        }
    }
}

fn slide_drawer(mut drawer_query: Query<(&mut Transform, &LetterDrawer)>, time: Res<Time>) {
    if let Ok((mut transform, drawer)) = drawer_query.get_single_mut() {
        let target_position = drawer.target_position;
        let direction = (target_position - transform.translation).normalize();
        let adjustment = direction + time.delta_seconds();
        if target_position.distance(transform.translation) > adjustment.length() {
            transform.translation += direction + time.delta_seconds();
        } else {
            transform.translation = target_position;
        }
    }
}
