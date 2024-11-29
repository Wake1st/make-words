use bevy::prelude::*;

use crate::{demo::letters::letter_loader::LetterList, screens::Screen, AppSet};

use super::interaction::{CloseDrawer, OpenDrawer};

const UI_BASE_WIDTH: f32 = 288.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_drawer).add_systems(
        Update,
        ((show_drawer, hide_drawer), slide_drawer)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component)]
pub struct LetterDrawer {
    pub target_position: Vec2,
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
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
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
                            flex_direction: FlexDirection::Column,
                            height: Val::Px(UI_BASE_WIDTH),
                            width: Val::Percent(100.),
                            ..Default::default()
                        },
                        ..Default::default()
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
                                    width: Val::Px(256.),
                                    height: Val::Px(256.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                transform: Transform::from_scale(Vec2::splat(2.0).extend(0.0)),
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
            drawer.target_position = transform.translation.xy() + Vec2::new(UI_BASE_WIDTH, 0.0);
        }
    }
}

fn hide_drawer(
    mut close_drawer: EventReader<CloseDrawer>,
    mut drawer_query: Query<(&mut LetterDrawer, &Transform)>,
) {
    for _ in close_drawer.read() {
        if let Ok((mut drawer, transform)) = drawer_query.get_single_mut() {
            drawer.target_position = transform.translation.xy() - Vec2::new(UI_BASE_WIDTH, 0.0);
        }
    }
}

fn slide_drawer(mut drawer_query: Query<(&mut Transform, &LetterDrawer)>, time: Res<Time>) {
    if let Ok((mut transform, drawer)) = drawer_query.get_single_mut() {
        let direction = (drawer.target_position.extend(0.0) - transform.translation).normalize();
        let adjustment = direction + time.delta_seconds();
        if drawer
            .target_position
            .extend(0.0)
            .distance(transform.translation)
            > adjustment.length()
        {
            transform.translation += direction + time.delta_seconds();
        } else {
            transform.translation = drawer.target_position.extend(0.0);
        }
    }
}
