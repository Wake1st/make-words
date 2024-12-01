use bevy::prelude::*;

use crate::{
    demo::letters::letter_loader::{load_letters, LetterList},
    screens::Screen,
    AppSet,
};

use super::interaction::{CloseDrawer, OpenDrawer};

const DRAWER_CLOSED: f32 = -100.0;
const DRAWER_OPEN: f32 = 0.0;
const DRAW_SLIDE_SPEED: f32 = 120.0;

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.0, 0.1, 0.2, 0.8);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_drawer.after(load_letters))
        .add_systems(
            Update,
            ((show_drawer, hide_drawer), slide_drawer)
                .chain()
                .in_set(AppSet::Update),
        );
}

#[derive(Component)]
pub struct LetterDrawer {
    pub current_position: f32,
    pub opening: bool,
    pub moving: bool,
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
                    position_type: PositionType::Absolute,
                    top: Val::Percent(DRAWER_CLOSED),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..default()
            },
            LetterDrawer {
                current_position: DRAWER_CLOSED,
                opening: false,
                moving: false,
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
                        background_color: BackgroundColor::from(BACKGROUND_COLOR),
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
    mut drawer_query: Query<&mut LetterDrawer>,
) {
    for _ in open_drawer.read() {
        if let Ok(mut drawer) = drawer_query.get_single_mut() {
            drawer.current_position = DRAWER_OPEN;
            drawer.opening = true;
            drawer.moving = true;
        }
    }
}

fn hide_drawer(
    mut close_drawer: EventReader<CloseDrawer>,
    mut drawer_query: Query<&mut LetterDrawer>,
) {
    for _ in close_drawer.read() {
        if let Ok(mut drawer) = drawer_query.get_single_mut() {
            drawer.current_position = DRAWER_CLOSED;
            drawer.opening = false;
            drawer.moving = true;
        }
    }
}

fn slide_drawer(mut drawer_query: Query<(&mut Style, &mut LetterDrawer)>, time: Res<Time>) {
    if let Ok((mut style, mut drawer)) = drawer_query.get_single_mut() {
        //  don't process if nothing is happening
        if !drawer.moving {
            return;
        }

        let adjustment = DRAW_SLIDE_SPEED * time.delta_seconds();

        if drawer.opening {
            let new_position = drawer.current_position + adjustment;
            info!(
                "adjustment: {:?}\tnew position: {:?}",
                adjustment, new_position
            );
            if (DRAWER_OPEN - new_position) > adjustment {
                style.top = Val::Percent(new_position);
                drawer.current_position = new_position;
            } else {
                style.top = Val::Percent(DRAWER_OPEN);
                drawer.current_position = DRAWER_OPEN;
                drawer.moving = false;
            }
        } else {
            let new_position = drawer.current_position - adjustment;
            info!(
                "adjustment: {:?}\tnew position: {:?}",
                adjustment, new_position
            );
            if (new_position - DRAWER_CLOSED) > adjustment {
                style.top = Val::Percent(new_position);
                drawer.current_position = new_position;
            } else {
                style.top = Val::Percent(DRAWER_CLOSED);
                drawer.current_position = DRAWER_CLOSED;
                drawer.moving = false;
            }
        }
    }
}
